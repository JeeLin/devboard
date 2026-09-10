use crate::db::connection::DbConn;
use crate::error::{DevBoardError, Result};
use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{params, Row};

#[derive(Debug, Clone, PartialEq)]
pub enum TaskType {
    Requirement,
    Task,
    Bug,
}

impl TaskType {
    pub fn as_str(&self) -> &str {
        match self {
            TaskType::Requirement => "requirement",
            TaskType::Task => "task",
            TaskType::Bug => "bug",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "requirement" | "req" => Ok(TaskType::Requirement),
            "task" | "tsk" => Ok(TaskType::Task),
            "bug" => Ok(TaskType::Bug),
            _ => Err(DevBoardError::InvalidInput(format!(
                "Unknown task type: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Review,
    Done,
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Review => "review",
            TaskStatus::Done => "done",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "todo" | "pending" => Ok(TaskStatus::Todo),
            "in_progress" | "active" => Ok(TaskStatus::InProgress),
            "review" => Ok(TaskStatus::Review),
            "done" | "completed" => Ok(TaskStatus::Done),
            _ => Err(DevBoardError::InvalidInput(format!(
                "Unknown status: {}",
                s
            ))),
        }
    }

    pub fn can_transition_to(&self, target: &TaskStatus) -> bool {
        use TaskStatus::*;
        matches!(
            (self, target),
            (Todo, InProgress)
                | (InProgress, Review)
                | (Review, Done)
                | (Review, InProgress)
                | (Done, Todo)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl TaskPriority {
    pub fn as_str(&self) -> &str {
        match self {
            TaskPriority::Low => "low",
            TaskPriority::Normal => "normal",
            TaskPriority::High => "high",
            TaskPriority::Urgent => "urgent",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "low" => Ok(TaskPriority::Low),
            "normal" | "" => Ok(TaskPriority::Normal),
            "high" => Ok(TaskPriority::High),
            "urgent" => Ok(TaskPriority::Urgent),
            _ => Err(DevBoardError::InvalidInput(format!(
                "Unknown priority: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Actor {
    Ai,
    Human,
}

impl Actor {
    pub fn as_str(&self) -> &str {
        match self {
            Actor::Ai => "ai",
            Actor::Human => "human",
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "ai" => Ok(Actor::Ai),
            "human" => Ok(Actor::Human),
            _ => Err(DevBoardError::InvalidInput(format!("Unknown actor: {}", s))),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub milestone_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub task_type: TaskType,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub assignee: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub actor: Actor,
    pub steps_to_reproduce: Option<String>,
    pub severity: Option<String>,
    pub environment: Option<String>,
    pub user_story: Option<String>,
    pub acceptance_criteria: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub total_time_seconds: i64,
}

impl Task {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let type_str: String = row.get(5)?;
        let status_str: String = row.get(9)?;
        let priority_str: String = row.get(10)?;
        let actor_str: String = row.get(14)?;
        let due_date_str: Option<String> = row.get(8)?;
        let created_at_str: String = row.get(17)?;
        let updated_at_str: String = row.get(18)?;

        Ok(Self {
            id: row.get(0)?,
            project_id: row.get(1)?,
            milestone_id: row.get(2)?,
            parent_id: row.get(3)?,
            task_type: TaskType::from_str(&type_str).unwrap_or(TaskType::Task),
            title: row.get(4)?,
            description: row.get(6)?,
            status: TaskStatus::from_str(&status_str).unwrap_or(TaskStatus::Todo),
            priority: TaskPriority::from_str(&priority_str).unwrap_or(TaskPriority::Normal),
            assignee: row.get(7)?,
            due_date: due_date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            actor: Actor::from_str(&actor_str).unwrap_or(Actor::Human),
            steps_to_reproduce: row.get(11)?,
            severity: row.get(12)?,
            environment: row.get(13)?,
            user_story: row.get(15)?,
            acceptance_criteria: row.get(16)?,
            created_at: NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default(),
            updated_at: NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default(),
            total_time_seconds: row.get(19)?,
        })
    }
}

const TASK_COLUMNS: &str = "id, project_id, milestone_id, parent_id, type, title, description, assignee, due_date, status, priority, steps_to_reproduce, severity, environment, actor, user_story, acceptance_criteria, created_at, updated_at, total_time_seconds";

pub fn list_tasks(
    conn: &DbConn,
    project_id: Option<i64>,
    task_type: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<Task>> {
    let mut conditions = vec![];
    let mut values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

    if let Some(pid) = project_id {
        conditions.push(format!("project_id = ?{}", values.len() + 1));
        values.push(Box::new(pid));
    }
    if let Some(t) = task_type {
        conditions.push(format!("type = ?{}", values.len() + 1));
        values.push(Box::new(t.to_string()));
    }
    if let Some(s) = status {
        conditions.push(format!("status = ?{}", values.len() + 1));
        values.push(Box::new(s.to_string()));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT {} FROM tasks {} ORDER BY id",
        TASK_COLUMNS, where_clause
    );
    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|p| p.as_ref()).collect();
    let tasks = stmt
        .query_map(params_refs.as_slice(), Task::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(tasks)
}

pub fn get_task(conn: &DbConn, id: i64) -> Result<Task> {
    let sql = format!("SELECT {} FROM tasks WHERE id = ?1", TASK_COLUMNS);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(params![id])?;
    match rows.next()? {
        Some(row) => Ok(Task::from_row(row)?),
        None => Err(DevBoardError::NotFound(format!("Task {}", id))),
    }
}

pub fn create_task(
    conn: &DbConn,
    project_id: i64,
    milestone_id: Option<i64>,
    task_type: &str,
    title: &str,
    description: Option<&str>,
    actor: Option<&str>,
) -> Result<Task> {
    let actor_val = actor.unwrap_or("human");
    conn.execute(
        "INSERT INTO tasks (project_id, milestone_id, type, title, description, actor) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![project_id, milestone_id, task_type, title, description, actor_val],
    )?;
    let id = conn.last_insert_rowid();
    get_task(conn, id)
}

pub fn update_task(
    conn: &DbConn,
    id: i64,
    title: Option<&str>,
    description: Option<&str>,
    status: Option<&str>,
    priority: Option<&str>,
    assignee: Option<&str>,
) -> Result<Task> {
    if let Some(t) = title {
        conn.execute(
            "UPDATE tasks SET title = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![t, id],
        )?;
    }
    if let Some(d) = description {
        conn.execute(
            "UPDATE tasks SET description = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![d, id],
        )?;
    }
    if let Some(s) = status {
        conn.execute(
            "UPDATE tasks SET status = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![s, id],
        )?;
    }
    if let Some(p) = priority {
        conn.execute(
            "UPDATE tasks SET priority = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![p, id],
        )?;
    }
    if let Some(a) = assignee {
        conn.execute(
            "UPDATE tasks SET assignee = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![a, id],
        )?;
    }
    get_task(conn, id)
}

pub fn delete_task(conn: &DbConn, id: i64) -> Result<()> {
    let affected = conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(DevBoardError::NotFound(format!("Task {}", id)));
    }
    Ok(())
}

pub fn transition_status(conn: &DbConn, id: i64, new_status: &str) -> Result<Task> {
    let task = get_task(conn, id)?;
    let target = TaskStatus::from_str(new_status)?;

    if !task.status.can_transition_to(&target) {
        return Err(DevBoardError::InvalidStateTransition {
            from: task.status.as_str().to_string(),
            to: target.as_str().to_string(),
        });
    }

    conn.execute(
        "UPDATE tasks SET status = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
        params![target.as_str(), id],
    )?;
    get_task(conn, id)
}
