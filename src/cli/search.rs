use crate::db::{get_connection, run_migrations};
use crate::models::document;
use clap::Args;
use std::path::PathBuf;

fn get_db() -> crate::error::Result<(rusqlite::Connection, PathBuf)> {
    let db_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".devboard")
        .join("data.db");
    std::fs::create_dir_all(db_path.parent().unwrap())?;
    let conn = get_connection(&db_path)?;
    run_migrations(&conn)?;
    Ok((conn, db_path))
}

#[derive(Args)]
pub struct SearchArgs {
    /// Search query
    pub query: String,
}

pub fn handle(args: SearchArgs) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match document::search_documents_fts(&conn, &args.query) {
        Ok(docs) => {
            if docs.is_empty() {
                println!("No results found for '{}'", args.query);
            } else {
                println!("Search results for '{}':\n", args.query);
                for doc in &docs {
                    println!("[{}] {} (task: {})", doc.id, doc.title, doc.task_id);
                    println!("  Status: {} | Created: {}", doc.status, doc.created_at);
                    // Show first 100 chars of content
                    let preview = doc.content.chars().take(100).collect::<String>();
                    println!("  {}\n", preview);
                }
            }
        }
        Err(e) => eprintln!("Search error: {}", e),
    }
}
