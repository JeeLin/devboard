use crate::db::connection::DbConn;
use crate::error::Result;
use rusqlite::params;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
}

impl Tag {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            color: row.get("color")?,
        })
    }
}

#[allow(dead_code)]
pub fn create_tag(conn: &DbConn, name: &str, color: &str) -> Result<Tag> {
    conn.execute(
        "INSERT INTO tags (name, color) VALUES (?1, ?2)",
        params![name, color],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Tag {
        id,
        name: name.to_string(),
        color: color.to_string(),
    })
}

#[allow(dead_code)]
pub fn list_tags(conn: &DbConn) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare("SELECT id, name, color FROM tags ORDER BY name")?;
    let tags = stmt
        .query_map([], Tag::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(tags)
}

#[allow(dead_code)]
pub fn delete_tag(conn: &DbConn, id: i64) -> Result<()> {
    conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
    Ok(())
}

#[allow(dead_code)]
pub fn add_tag_to_task(conn: &DbConn, task_id: i64, tag_id: i64) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO task_tags (task_id, tag_id) VALUES (?1, ?2)",
        params![task_id, tag_id],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn remove_tag_from_task(conn: &DbConn, task_id: i64, tag_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM task_tags WHERE task_id = ?1 AND tag_id = ?2",
        params![task_id, tag_id],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn list_task_tags(conn: &DbConn, task_id: i64) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, t.color FROM tags t 
         JOIN task_tags tt ON t.id = tt.tag_id 
         WHERE tt.task_id = ?1 ORDER BY t.name",
    )?;
    let tags = stmt
        .query_map(params![task_id], Tag::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(tags)
}

#[allow(dead_code)]
pub fn list_tasks_by_tag(conn: &DbConn, tag_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare("SELECT task_id FROM task_tags WHERE tag_id = ?1")?;
    let task_ids = stmt
        .query_map(params![tag_id], |row| row.get(0))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(task_ids)
}

#[allow(dead_code)]
pub fn add_tag_to_document(conn: &DbConn, document_id: i64, tag_id: i64) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO document_tags (document_id, tag_id) VALUES (?1, ?2)",
        params![document_id, tag_id],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn remove_tag_from_document(conn: &DbConn, document_id: i64, tag_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM document_tags WHERE document_id = ?1 AND tag_id = ?2",
        params![document_id, tag_id],
    )?;
    Ok(())
}

#[allow(dead_code)]
pub fn list_document_tags(conn: &DbConn, document_id: i64) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, t.color FROM tags t 
         JOIN document_tags dt ON t.id = dt.tag_id 
         WHERE dt.document_id = ?1 ORDER BY t.name",
    )?;
    let tags = stmt
        .query_map(params![document_id], Tag::from_row)?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(tags)
}
