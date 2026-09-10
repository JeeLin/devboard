use crate::error::Result;
use rusqlite::Connection;
use std::path::Path;

pub type DbConn = Connection;

pub fn get_connection(db_path: &Path) -> Result<DbConn> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    Ok(conn)
}

#[allow(dead_code)]
pub fn get_memory_connection() -> Result<DbConn> {
    let conn = Connection::open_in_memory()?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;
    Ok(conn)
}
