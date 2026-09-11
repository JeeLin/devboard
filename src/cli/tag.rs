use clap::Subcommand;
use crate::db::{get_connection, run_migrations};
use crate::models::tag;
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
pub enum TagArgs {
    /// Create a new tag
    Add {
        /// Tag name
        name: String,
        /// Tag color (hex, default: #4a9eff)
        #[arg(long, default_value = "#4a9eff")]
        color: String,
    },
    /// List all tags
    List,
    /// Delete a tag
    Delete {
        /// Tag ID
        id: i64,
    },
}

pub fn handle(args: TagArgs) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => { eprintln!("Error: {}", e); std::process::exit(1); }
    };
    match args {
        TagArgs::Add { name, color } => {
            match tag::create_tag(&conn, &name, &color) {
                Ok(t) => println!("Created tag '{}' (id: {}, color: {})", t.name, t.id, t.color),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        TagArgs::List => {
            match tag::list_tags(&conn) {
                Ok(tags) => {
                    if tags.is_empty() {
                        println!("No tags found.");
                    } else {
                        println!("{:<5} {:<20} {:<10}", "ID", "Name", "Color");
                        println!("{}", "-".repeat(35));
                        for t in &tags {
                            println!("{:<5} {:<20} {:<10}", t.id, t.name, t.color);
                        }
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        TagArgs::Delete { id } => {
            match tag::delete_tag(&conn, id) {
                Ok(()) => println!("Deleted tag {}", id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
