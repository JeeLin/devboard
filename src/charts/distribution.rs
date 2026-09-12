use crate::db::{get_connection, run_migrations};
use crate::models::time_entry;
use std::path::PathBuf;

pub fn generate_distribution(date: &str) -> Result<String, Box<dyn std::error::Error>> {
    let db_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".devboard")
        .join("data.db");
    let conn = get_connection(&db_path)?;
    run_migrations(&conn)?;

    let date_naive = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
    let stats = time_entry::get_daily_time_summary(&conn, date_naive)?;

    let mut svg =
        String::from("<svg width=\"400\" height=\"300\" xmlns=\"http://www.w3.org/2000/svg\">\n");
    svg.push_str("<style>\n");
    svg.push_str("  .slice { font-family: monospace; font-size: 12px; }\n");
    svg.push_str("</style>\n");

    let total: i64 = stats.iter().map(|(_, s)| s).sum();
    let mut x = 50;
    let colors = ["#4a9eff", "#ff6b6b", "#00cc00", "#ffa500", "#9b59b6"];

    for (i, (actor, secs)) in stats.iter().enumerate() {
        let width = if total > 0 {
            (*secs as f64 / total as f64 * 300.0) as i32
        } else {
            0
        };
        let color = colors[i % colors.len()];

        svg.push_str(&format!(
            "<rect x=\"{}\" y=\"50\" width=\"{}\" height=\"50\" fill=\"{}\" />\n",
            x, width, color
        ));
        svg.push_str(&format!(
            "<text class=\"slice\" x=\"{}\" y=\"130\">{}: {:.1}h</text>\n",
            x,
            actor,
            *secs as f64 / 3600.0
        ));
        x += width + 10;
    }

    svg.push_str("</svg>");
    Ok(svg)
}
