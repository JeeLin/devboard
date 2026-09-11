use crate::db::{get_connection, run_migrations};
use crate::models::task::{self, TaskStatus};
use std::path::PathBuf;

pub fn generate_gantt(_milestone_id: i64) -> Result<String, Box<dyn std::error::Error>> {
    let db_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".devboard")
        .join("data.db");
    let conn = get_connection(&db_path)?;
    run_migrations(&conn)?;

    let tasks = task::list_tasks(&conn, None, None, None)?;
    
    let mut svg = String::from("<svg width=\"800\" height=\"600\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg.push_str("<style>\n");
    svg.push_str("  .task { font-family: monospace; font-size: 12px; }\n");
    svg.push_str("  .bar { opacity: 0.8; }\n");
    svg.push_str("</style>\n");
    
    let mut y = 30;
    for (i, task) in tasks.iter().enumerate() {
        let color = match task.status {
            TaskStatus::Todo => "#cccccc",
            TaskStatus::InProgress => "#4a9eff",
            TaskStatus::Review => "#ffa500",
            TaskStatus::Done => "#00cc00",
        };
        
        let width = 100 + (i as i32 * 20).min(400);
        svg.push_str(&format!(
            "<rect class=\"bar\" x=\"200\" y=\"{}\" width=\"{}\" height=\"20\" fill=\"{}\" />\n",
            y, width, color
        ));
        svg.push_str(&format!(
            "<text class=\"task\" x=\"10\" y=\"{}\">{}</text>\n",
            y + 15, task.title
        ));
        y += 30;
    }
    
    svg.push_str("</svg>");
    Ok(svg)
}
