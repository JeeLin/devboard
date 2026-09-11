use clap::Subcommand;
use crate::db::{get_connection, run_migrations};
use crate::models::document;
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

#[derive(Subcommand)]
pub enum DocArgs {
    /// Add a document to a task
    Add {
        #[arg(short, long)]
        task: i64,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        file: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
    /// List documents for a task
    List {
        #[arg(short, long)]
        task: i64,
    },
    /// Get a document by ID
    Get {
        id: i64,
    },
    /// Update document status
    Status {
        id: i64,
        status: String,
    },
}

pub fn handle(args: DocArgs) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => { eprintln!("Error: {}", e); std::process::exit(1); }
    };
    match args {
        DocArgs::Add { task, title, file, content } => {
            let doc_content = if let Some(file_path) = file {
                std::fs::read_to_string(&file_path).unwrap_or_default()
            } else {
                content.unwrap_or_default()
            };
            match document::create_document(&conn, task, &title, &doc_content) {
                Ok(doc) => println!("Created document '{}' (id: {}, task: {})", doc.title, doc.id, doc.task_id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        DocArgs::List { task } => {
            match document::list_documents_by_task(&conn, task) {
                Ok(docs) => {
                    if docs.is_empty() {
                        println!("No documents found for task {}", task);
                    } else {
                        println!("Documents for task {}:", task);
                        for doc in &docs {
                            println!("  [{}] {} ({})", doc.id, doc.title, doc.status);
                        }
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        DocArgs::Get { id } => {
            match document::get_document(&conn, id) {
                Ok(doc) => {
                    println!("Document: {} (id: {})", doc.title, doc.id);
                    println!("Status: {} | Task: {}", doc.status, doc.task_id);
                    println!("\n{}", doc.content);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        DocArgs::Status { id, status } => {
            match document::update_document_status(&conn, id, &status) {
                Ok(doc) => println!("Updated document '{}' status to {}", doc.title, doc.status),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
