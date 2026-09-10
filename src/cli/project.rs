use crate::db::{get_connection, run_migrations};
use crate::models::project;
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
pub enum ProjectArgs {
    /// List all projects
    List,
    /// Create a new project
    Add {
        /// Project name
        name: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Update a project
    Update {
        /// Project ID
        id: i64,
        /// New name
        #[arg(short, long)]
        name: Option<String>,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Delete a project
    Delete {
        /// Project ID
        id: i64,
    },
}

pub fn handle(args: ProjectArgs) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match args {
        ProjectArgs::List => match project::list_projects(&conn) {
            Ok(projects) => {
                if projects.is_empty() {
                    println!("No projects found.");
                    return;
                }
                println!("{:<5} {:<30} {:<20}", "ID", "Name", "Created");
                println!("{}", "-".repeat(55));
                for p in &projects {
                    println!("{:<5} {:<30} {:<20}", p.id, p.name, p.created_at);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        ProjectArgs::Add { name, description } => {
            match project::create_project(&conn, &name, description.as_deref()) {
                Ok(p) => println!("Created project: {} (id: {})", p.name, p.id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        ProjectArgs::Update {
            id,
            name,
            description,
        } => match project::update_project(&conn, id, name.as_deref(), description.as_deref()) {
            Ok(p) => println!("Updated project: {} (id: {})", p.name, p.id),
            Err(e) => eprintln!("Error: {}", e),
        },
        ProjectArgs::Delete { id } => match project::delete_project(&conn, id) {
            Ok(()) => println!("Deleted project {}", id),
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
