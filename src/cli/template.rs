use crate::db::{get_connection, run_migrations};
use crate::models::{task, template};
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
pub enum TemplateArgs {
    /// Create a new task template
    Add {
        /// Template name
        name: String,
        /// Task type (requirement/task/bug)
        #[arg(short = 'T', long, default_value = "task")]
        task_type: String,
        /// Title template (supports {name} variable)
        #[arg(short, long)]
        title: String,
        /// Description template
        #[arg(short, long)]
        description: String,
        /// Default priority
        #[arg(long, default_value = "normal")]
        priority: String,
        /// Default actor
        #[arg(long, default_value = "human")]
        actor: String,
    },
    /// List all templates
    List,
    /// Delete a template
    Delete {
        /// Template ID
        id: i64,
    },
    /// Create a task from template
    Create {
        /// Template ID
        #[arg(short = 't', long)]
        template: i64,
        /// Project ID
        #[arg(short = 'p', long)]
        project: i64,
        /// Task name (replaces {name} in template)
        #[arg(short, long)]
        name: String,
    },
}

pub fn handle(args: TemplateArgs) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    match args {
        TemplateArgs::Add {
            name,
            task_type,
            title,
            description,
            priority,
            actor,
        } => {
            match template::create_template(
                &conn,
                &name,
                &task_type,
                &title,
                &description,
                &priority,
                &actor,
            ) {
                Ok(t) => println!("Created template '{}' (id: {})", t.name, t.id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        TemplateArgs::List => match template::list_templates(&conn) {
            Ok(templates) => {
                if templates.is_empty() {
                    println!("No templates found.");
                } else {
                    println!(
                        "{:<5} {:<20} {:<10} {:<30}",
                        "ID", "Name", "Type", "Title Template"
                    );
                    println!("{}", "-".repeat(65));
                    for t in &templates {
                        println!(
                            "{:<5} {:<20} {:<10} {:<30}",
                            t.id, t.name, t.task_type, t.title_template
                        );
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        TemplateArgs::Delete { id } => match template::delete_template(&conn, id) {
            Ok(()) => println!("Deleted template {}", id),
            Err(e) => eprintln!("Error: {}", e),
        },
        TemplateArgs::Create {
            template: tmpl_id,
            project,
            name,
        } => match template::get_template(&conn, tmpl_id) {
            Ok(tmpl) => {
                let title = tmpl.title_template.replace("{name}", &name);
                let description = tmpl.description_template.replace("{name}", &name);
                match task::create_task(
                    &conn,
                    project,
                    None,
                    &tmpl.task_type,
                    &title,
                    Some(&description),
                    Some(&tmpl.default_actor),
                ) {
                    Ok(t) => println!(
                        "Created task '{}' (id: {}) from template '{}'",
                        t.title, t.id, tmpl.name
                    ),
                    Err(e) => eprintln!("Error creating task: {}", e),
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
