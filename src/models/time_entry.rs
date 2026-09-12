use crate::db::connection::DbConn;
use crate::error::{DevBoardError, Result};
use chrono::NaiveDateTime;
use rusqlite::{params, Row};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TimeEntry {
    pub id: i64,
    pub task_id: i64,
    pub actor: String,
    pub duration: i64,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
}

impl TimeEntry {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let start_time_str: Option<String> = row.get(4)?;
        let end_time_str: Option<String> = row.get(5)?;
        let created_at_str: String = row.get(7)?;
        Ok(Self {
            id: row.get(0)?,
            task_id: row.get(1)?,
            actor: row.get(2)?,
            duration: row.get(3)?,
            start_time: start_time_str
                .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok()),
            end_time: end_time_str
                .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok()),
            note: row.get(6)?,
            created_at: NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_default(),
        })
    }
}

pub fn list_time_entries(conn: &DbConn, task_id: Option<i64>) -> Result<Vec<TimeEntry>> {
    let (sql, params_vec): (&str, Vec<Box<dyn rusqlite::types::ToSql>>) = match task_id {
        Some(tid) => (
            "SELECT id, task_id, actor, duration, start_time, end_time, note, created_at FROM time_entries WHERE task_id = ?1 ORDER BY id",
            vec![Box::new(tid)],
        ),
        None => (
            "SELECT id, task_id, actor, duration, start_time, end_time, note, created_at FROM time_entries ORDER BY id",
            vec![],
        ),
    };
    let mut stmt = conn.prepare(sql)?;
    let params_refs: Vec<&dyn rusqlite::types::ToSql> =
        params_vec.iter().map(|p| p.as_ref()).collect();
    let entries = stmt
        .query_map(params_refs.as_slice(), TimeEntry::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(entries)
}

pub fn create_time_entry(
    conn: &DbConn,
    task_id: i64,
    actor: &str,
    duration: i64,
    note: Option<&str>,
) -> Result<TimeEntry> {
    conn.execute(
        "INSERT INTO time_entries (task_id, actor, duration, note) VALUES (?1, ?2, ?3, ?4)",
        params![task_id, actor, duration, note],
    )?;
    let id = conn.last_insert_rowid();
    let mut stmt = conn.prepare(
        "SELECT id, task_id, actor, duration, start_time, end_time, note, created_at FROM time_entries WHERE id = ?1"
    )?;
    let mut rows = stmt.query(params![id])?;
    match rows.next()? {
        Some(row) => Ok(TimeEntry::from_row(row)?),
        None => Err(DevBoardError::NotFound(format!("TimeEntry {}", id))),
    }
}

pub fn get_time_stats(conn: &DbConn) -> Result<Vec<(String, i64)>> {
    let mut stmt = conn.prepare(
        "SELECT actor, SUM(duration) as total FROM time_entries GROUP BY actor ORDER BY actor",
    )?;
    let stats = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(stats)
}

/// Get daily time summary grouped by actor for a specific date.
pub fn get_daily_time_summary(
    conn: &DbConn,
    date: chrono::NaiveDate,
) -> Result<Vec<(String, i64)>> {
    let start = format!("{} 00:00:00", date);
    let end = format!("{} 23:59:59", date);
    let mut stmt = conn.prepare(
        "SELECT actor, SUM(duration) as total FROM time_entries WHERE created_at BETWEEN ?1 AND ?2 GROUP BY actor ORDER BY actor",
    )?;
    let stats = stmt
        .query_map(params![start, end], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(stats)
}

pub fn get_weekly_summary(
    conn: &DbConn,
    start_date: chrono::NaiveDate,
) -> Result<Vec<(String, i64)>> {
    let end_date = start_date + chrono::Duration::days(7);
    let mut stmt = conn.prepare(
        "SELECT actor, SUM(duration) as total FROM time_entries 
         WHERE date(created_at) >= ?1 AND date(created_at) < ?2 
         GROUP BY actor ORDER BY actor",
    )?;
    let start_str = start_date.format("%Y-%m-%d").to_string();
    let end_str = end_date.format("%Y-%m-%d").to_string();
    let stats = stmt
        .query_map(rusqlite::params![start_str, end_str], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(stats)
}

pub fn get_monthly_summary(conn: &DbConn, year: i32, month: u32) -> Result<Vec<(String, i64)>> {
    let start_date = chrono::NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| DevBoardError::InvalidInput("Invalid date".to_string()))?;
    let end_date = if month == 12 {
        chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        chrono::NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .ok_or_else(|| DevBoardError::InvalidInput("Invalid date".to_string()))?;

    let mut stmt = conn.prepare(
        "SELECT actor, SUM(duration) as total FROM time_entries 
         WHERE date(created_at) >= ?1 AND date(created_at) < ?2 
         GROUP BY actor ORDER BY actor",
    )?;
    let start_str = start_date.format("%Y-%m-%d").to_string();
    let end_str = end_date.format("%Y-%m-%d").to_string();
    let stats = stmt
        .query_map(rusqlite::params![start_str, end_str], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(stats)
}
