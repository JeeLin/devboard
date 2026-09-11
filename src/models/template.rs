use crate::db::connection::DbConn;
use crate::error::Result;
use rusqlite::params;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TaskTemplate {
    pub id: i64,
    pub name: String,
    pub task_type: String,
    pub title_template: String,
    pub description_template: String,
    pub default_priority: String,
    pub default_actor: String,
}

impl TaskTemplate {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            task_type: row.get("task_type")?,
            title_template: row.get("title_template")?,
            description_template: row.get("description_template")?,
            default_priority: row.get("default_priority")?,
            default_actor: row.get("default_actor")?,
        })
    }
}

#[allow(dead_code)]
pub fn create_template(
    conn: &DbConn,
    name: &str,
    task_type: &str,
    title_template: &str,
    description_template: &str,
    default_priority: &str,
    default_actor: &str,
) -> Result<TaskTemplate> {
    conn.execute(
        "INSERT INTO task_templates (name, task_type, title_template, description_template, default_priority, default_actor) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![name, task_type, title_template, description_template, default_priority, default_actor],
    )?;
    let id = conn.last_insert_rowid();
    Ok(TaskTemplate {
        id,
        name: name.to_string(),
        task_type: task_type.to_string(),
        title_template: title_template.to_string(),
        description_template: description_template.to_string(),
        default_priority: default_priority.to_string(),
        default_actor: default_actor.to_string(),
    })
}

#[allow(dead_code)]
pub fn list_templates(conn: &DbConn) -> Result<Vec<TaskTemplate>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, task_type, title_template, description_template, default_priority, default_actor 
         FROM task_templates ORDER BY name"
    )?;
    let templates = stmt
        .query_map([], TaskTemplate::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(templates)
}

#[allow(dead_code)]
pub fn get_template(conn: &DbConn, id: i64) -> Result<TaskTemplate> {
    conn.query_row(
        "SELECT id, name, task_type, title_template, description_template, default_priority, default_actor 
         FROM task_templates WHERE id = ?1",
        params![id],
        TaskTemplate::from_row,
    )
    .map_err(|e| e.into())
}

#[allow(dead_code)]
pub fn delete_template(conn: &DbConn, id: i64) -> Result<()> {
    conn.execute("DELETE FROM task_templates WHERE id = ?1", params![id])?;
    Ok(())
}
