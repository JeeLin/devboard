use crate::db::connection::DbConn;
use crate::error::{DevBoardError, Result};
use chrono::{NaiveDate, NaiveDateTime};
use rusqlite::{params, Row};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Milestone {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub version: Option<String>,
    pub target_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: NaiveDateTime,
}

impl Milestone {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let target_date_str: Option<String> = row.get(4)?;
        let created_at_str: String = row.get(6)?;
        Ok(Self {
            id: row.get(0)?,
            project_id: row.get(1)?,
            name: row.get(2)?,
            version: row.get(3)?,
            target_date: target_date_str
                .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            status: row.get(5)?,
            created_at: NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default(),
        })
    }
}

pub fn list_milestones(conn: &DbConn, project_id: Option<i64>) -> Result<Vec<Milestone>> {
    let (sql, params_vec): (&str, Vec<Box<dyn rusqlite::types::ToSql>>) = match project_id {
        Some(pid) => (
            "SELECT id, project_id, name, version, target_date, status, created_at FROM milestones WHERE project_id = ?1 ORDER BY id",
            vec![Box::new(pid)],
        ),
        None => (
            "SELECT id, project_id, name, version, target_date, status, created_at FROM milestones ORDER BY id",
            vec![],
        ),
    };
    let mut stmt = conn.prepare(sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> =
        params_vec.iter().map(|p| p.as_ref()).collect();
    let milestones = stmt
        .query_map(params_refs.as_slice(), Milestone::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(milestones)
}

pub fn get_milestone(conn: &DbConn, id: i64) -> Result<Milestone> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, name, version, target_date, status, created_at FROM milestones WHERE id = ?1"
    )?;
    let mut rows = stmt.query(params![id])?;
    match rows.next()? {
        Some(row) => Ok(Milestone::from_row(row)?),
        None => Err(DevBoardError::NotFound(format!("Milestone {}", id))),
    }
}

pub fn create_milestone(
    conn: &DbConn,
    project_id: i64,
    name: &str,
    version: Option<&str>,
    target_date: Option<&str>,
) -> Result<Milestone> {
    conn.execute(
        "INSERT INTO milestones (project_id, name, version, target_date) VALUES (?1, ?2, ?3, ?4)",
        params![project_id, name, version, target_date],
    )?;
    let id = conn.last_insert_rowid();
    get_milestone(conn, id)
}

pub fn update_milestone(
    conn: &DbConn,
    id: i64,
    name: Option<&str>,
    status: Option<&str>,
) -> Result<Milestone> {
    if let Some(n) = name {
        conn.execute(
            "UPDATE milestones SET name = ?1 WHERE id = ?2",
            params![n, id],
        )?;
    }
    if let Some(s) = status {
        conn.execute(
            "UPDATE milestones SET status = ?1 WHERE id = ?2",
            params![s, id],
        )?;
    }
    get_milestone(conn, id)
}

pub fn delete_milestone(conn: &DbConn, id: i64) -> Result<()> {
    let affected = conn.execute("DELETE FROM milestones WHERE id = ?1", params![id])?;
    if affected == 0 {
        return Err(DevBoardError::NotFound(format!("Milestone {}", id)));
    }
    Ok(())
}
