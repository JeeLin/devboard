use crate::db::{get_connection, run_migrations};
use crate::models::task;
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
pub enum TaskArgs {
    /// List tasks
    List {
        /// Filter by project ID
        #[arg(short, long)]
        project: Option<i64>,
        /// Filter by type (requirement/task/bug)
        #[arg(short = 'T', long)]
        task_type: Option<String>,
        /// Filter by status (todo/in_progress/review/done)
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Create a new task
    Add {
        /// Project ID
        #[arg(short, long)]
        project: i64,
        /// Task type (requirement/task/bug)
        #[arg(short = 'T', long, default_value = "task")]
        task_type: String,
        /// Task title
        title: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
        /// Actor (ai/human)
        #[arg(long)]
        actor: Option<String>,
    },
    /// Update a task
    Update {
        /// Task ID
        id: i64,
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        /// New status
        #[arg(short, long)]
        status: Option<String>,
        /// New priority
        #[arg(short, long)]
        priority: Option<String>,
        /// New assignee
        #[arg(short, long)]
        assignee: Option<String>,
    },
    /// Delete a task
    Delete {
        /// Task ID
        id: i64,
    },
    /// Change task status
    Status {
        /// Task ID
        id: i64,
        /// New status (todo/in_progress/review/done)
        new_status: String,
    },
}

pub fn handle(args: TaskArgs) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match args {
        TaskArgs::List {
            project,
            task_type,
            status,
        } => match task::list_tasks(&conn, project, task_type.as_deref(), status.as_deref()) {
            Ok(tasks) => {
                if tasks.is_empty() {
                    println!("No tasks found.");
                    return;
                }
                println!(
                    "{:<5} {:<8} {:<10} {:<12} {:<10} {:<30}",
                    "ID", "Type", "Status", "Priority", "Actor", "Title"
                );
                println!("{}", "-".repeat(75));
                for t in &tasks {
                    println!(
                        "{:<5} {:<8} {:<10} {:<12} {:<10} {:<30}",
                        t.id,
                        t.task_type.as_str(),
                        t.status.as_str(),
                        t.priority.as_str(),
                        t.actor.as_str(),
                        t.title,
                    );
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        TaskArgs::Add {
            project,
            task_type,
            title,
            description,
            actor,
        } => {
            match task::create_task(
                &conn,
                project,
                None,
                &task_type,
                &title,
                description.as_deref(),
                actor.as_deref(),
            ) {
                Ok(t) => println!(
                    "Created task: {} (id: {}, type: {})",
                    t.title,
                    t.id,
                    t.task_type.as_str()
                ),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        TaskArgs::Update {
            id,
            title,
            description,
            status,
            priority,
            assignee,
        } => {
            match task::update_task(
                &conn,
                id,
                title.as_deref(),
                description.as_deref(),
                status.as_deref(),
                priority.as_deref(),
                assignee.as_deref(),
            ) {
                Ok(t) => println!("Updated task: {} (id: {})", t.title, t.id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        TaskArgs::Delete { id } => match task::delete_task(&conn, id) {
            Ok(()) => println!("Deleted task {}", id),
            Err(e) => eprintln!("Error: {}", e),
        },
        TaskArgs::Status { id, new_status } => {
            match task::transition_status(&conn, id, &new_status) {
                Ok(t) => println!("Task {} status: {}", id, t.status.as_str()),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
