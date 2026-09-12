use crate::db::connection::DbConn;
use crate::error::{DevBoardError, Result};
use rusqlite::{params, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DocStatus {
    Draft,
    Review,
    Published,
    Archived,
}

impl std::fmt::Display for DocStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocStatus::Draft => write!(f, "Draft"),
            DocStatus::Review => write!(f, "Review"),
            DocStatus::Published => write!(f, "Published"),
            DocStatus::Archived => write!(f, "Archived"),
        }
    }
}

impl std::str::FromStr for DocStatus {
    type Err = DevBoardError;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "draft" => Ok(DocStatus::Draft),
            "review" => Ok(DocStatus::Review),
            "published" => Ok(DocStatus::Published),
            "archived" => Ok(DocStatus::Archived),
            _ => Err(DevBoardError::InvalidInput(format!(
                "Invalid doc status: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: i64,
    pub task_id: i64,
    pub title: String,
    pub content: String,
    pub status: DocStatus,
    pub created_at: String,
    pub updated_at: String,
}

impl Document {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let status_str: String = row.get(4)?;
        Ok(Document {
            id: row.get(0)?,
            task_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            status: status_str.parse().unwrap_or(DocStatus::Draft),
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }
}

pub const DOC_COLUMNS: &str = "id, task_id, title, content, status, created_at, updated_at";

pub fn create_document(
    conn: &DbConn,
    task_id: i64,
    title: &str,
    content: &str,
) -> Result<Document> {
    let mut stmt = conn.prepare(&format!(
        "INSERT INTO documents (task_id, title, content, status, created_at, updated_at) VALUES (?1, ?2, ?3, 'Draft', datetime('now'), datetime('now')) RETURNING {}",
        DOC_COLUMNS
    ))?;
    let doc = stmt.query_row(params![task_id, title, content], Document::from_row)?;
    Ok(doc)
}

pub fn get_document(conn: &DbConn, id: i64) -> Result<Document> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {} FROM documents WHERE id = ?1",
        DOC_COLUMNS
    ))?;
    let mut rows = stmt.query(params![id])?;
    match rows.next()? {
        Some(row) => Ok(Document::from_row(row)?),
        None => Err(DevBoardError::NotFound(format!("Document {}", id))),
    }
}

pub fn list_documents_by_task(conn: &DbConn, task_id: i64) -> Result<Vec<Document>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {} FROM documents WHERE task_id = ?1 ORDER BY created_at DESC",
        DOC_COLUMNS
    ))?;
    let docs = stmt
        .query_map(params![task_id], Document::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(docs)
}

pub fn update_document_status(conn: &DbConn, id: i64, status: &str) -> Result<Document> {
    let mut stmt = conn.prepare(&format!(
        "UPDATE documents SET status = ?1, updated_at = datetime('now') WHERE id = ?2 RETURNING {}",
        DOC_COLUMNS
    ))?;
    let doc = stmt.query_row(params![status, id], Document::from_row)?;
    Ok(doc)
}

pub fn search_documents(conn: &DbConn, query: &str) -> Result<Vec<Document>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {} FROM documents WHERE title LIKE ?1 OR content LIKE ?1 ORDER BY created_at DESC",
        DOC_COLUMNS
    ))?;
    let pattern = format!("%{}%", query);
    let docs = stmt
        .query_map(params![pattern], Document::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(docs)
}

pub fn search_documents_fts(conn: &DbConn, query: &str) -> Result<Vec<Document>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT d.{} FROM documents d 
         JOIN documents_fts fts ON d.id = fts.rowid 
         WHERE documents_fts MATCH ?1 
         ORDER BY rank LIMIT 20",
        DOC_COLUMNS
    ))?;
    let docs = stmt
        .query_map(params![query], Document::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(docs)
}

pub fn link_document_to_task(conn: &DbConn, document_id: i64, task_id: i64) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO document_links (document_id, task_id) VALUES (?1, ?2)",
        params![document_id, task_id],
    )?;
    Ok(())
}

pub fn unlink_document_from_task(conn: &DbConn, document_id: i64, task_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM document_links WHERE document_id = ?1 AND task_id = ?2",
        params![document_id, task_id],
    )?;
    Ok(())
}

pub fn list_documents_by_task_id(conn: &DbConn, task_id: i64) -> Result<Vec<Document>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT d.{} FROM documents d 
         JOIN document_links dl ON d.id = dl.document_id 
         WHERE dl.task_id = ?1 
         ORDER BY d.created_at DESC",
        DOC_COLUMNS
    ))?;
    let docs = stmt
        .query_map(params![task_id], Document::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(docs)
}

pub fn list_tasks_by_document(conn: &DbConn, document_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare("SELECT task_id FROM document_links WHERE document_id = ?1")?;
    let task_ids = stmt
        .query_map(params![document_id], |row| row.get(0))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(task_ids)
}
