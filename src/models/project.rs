use crate::db::connection::DbConn;
use crate::error::{DevBoardError, Result};
use chrono::NaiveDateTime;
use rusqlite::{params, Row};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Project {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let created_at_str: String = row.get(3)?;
        let updated_at_str: String = row.get(4)?;
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default(),
            updated_at: NaiveDateTime::parse_from_str(&updated_at_str, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default(),
        })
    }
}

pub fn list_projects(conn: &DbConn) -> Result<Vec<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at, updated_at FROM projects ORDER BY id",
    )?;
    let projects = stmt
        .query_map([], Project::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(projects)
}

pub fn get_project(conn: &DbConn, id: i64) -> Result<Project> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = ?1",
    )?;
    let mut rows = stmt.query(params![id])?;
    match rows.next()? {
        Some(row) => Ok(Project::from_row(row)?),
        None => Err(DevBoardError::NotFound(format!("Project {}", id))),
    }
}

pub fn create_project(conn: &DbConn, name: &str, description: Option<&str>) -> Result<Project> {
    conn.execute(
        "INSERT INTO projects (name, description) VALUES (?1, ?2)",
        params![name, description],
    )?;
    let id = conn.last_insert_rowid();
    get_project(conn, id)
}

pub fn update_project(
    conn: &DbConn,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<Project> {
    if let Some(n) = name {
        conn.execute(
            "UPDATE projects SET name = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![n, id],
        )?;
    }
    if let Some(d) = description {
        conn.execute(
            "UPDATE projects SET description = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
            params![d, id],
        )?;
    }
    get_project(conn, id)
}

pub fn delete_project(conn: &DbConn, id: i64) -> Result<()> {
    let affected = conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(DevBoardError::NotFound(format!("Project {}", id)));
    }
    Ok(())
}
