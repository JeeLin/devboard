use crate::db::{get_connection, run_migrations};
use crate::models::milestone;
use clap::Subcommand;
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
pub enum MilestoneArgs {
    /// List milestones
    List {
        /// Filter by project ID
        #[arg(short, long)]
        project: Option<i64>,
    },
    /// Create a new milestone
    Add {
        /// Project ID
        #[arg(short, long)]
        project: i64,
        /// Milestone name
        name: String,
        /// Version tag
        #[arg(short, long)]
        version: Option<String>,
        /// Target date (YYYY-MM-DD)
        #[arg(short, long)]
        target_date: Option<String>,
    },
    /// Update a milestone
    Update {
        /// Milestone ID
        id: i64,
        /// New name
        #[arg(short, long)]
        name: Option<String>,
        /// New status (active/completed)
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Delete a milestone
    Delete {
        /// Milestone ID
        id: i64,
    },
}

pub fn handle(args: MilestoneArgs) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match args {
        MilestoneArgs::List { project } => match milestone::list_milestones(&conn, project) {
            Ok(milestones) => {
                if milestones.is_empty() {
                    println!("No milestones found.");
                    return;
                }
                println!(
                    "{:<5} {:<20} {:<10} {:<15} {:<10}",
                    "ID", "Name", "Version", "Target", "Status"
                );
                println!("{}", "-".repeat(60));
                for m in &milestones {
                    println!(
                        "{:<5} {:<20} {:<10} {:<15} {:<10}",
                        m.id,
                        m.name,
                        m.version.as_deref().unwrap_or("-"),
                        m.target_date
                            .map(|d| d.to_string())
                            .unwrap_or_else(|| "-".to_string()),
                        m.status,
                    );
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        MilestoneArgs::Add {
            project,
            name,
            version,
            target_date,
        } => {
            match milestone::create_milestone(
                &conn,
                project,
                &name,
                version.as_deref(),
                target_date.as_deref(),
            ) {
                Ok(m) => println!("Created milestone: {} (id: {})", m.name, m.id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        MilestoneArgs::Update { id, name, status } => {
            match milestone::update_milestone(&conn, id, name.as_deref(), status.as_deref()) {
                Ok(m) => println!("Updated milestone: {} (id: {})", m.name, m.id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        MilestoneArgs::Delete { id } => match milestone::delete_milestone(&conn, id) {
            Ok(()) => println!("Deleted milestone {}", id),
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
