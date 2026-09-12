use crate::db::{get_connection, run_migrations};
use crate::models::time_entry;
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
pub enum TimeCommands {
    /// Log time entry for a task
    Log {
        /// Task ID
        #[arg(short, long)]
        task: i64,
        /// Time duration (e.g., "2h 30m", "90m", "1.5h")
        time: String,
        /// Optional note
        #[arg(short, long)]
        note: Option<String>,
        /// Actor (ai/human)
        #[arg(long)]
        actor: Option<String>,
    },
    /// Show time report
    Report {
        /// Filter by date (YYYY-MM-DD), defaults to today
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Show weekly time report
    Weekly {
        /// Start date (YYYY-MM-DD), defaults to this week's start
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Show monthly time report
    Monthly {
        /// Year (YYYY), defaults to current year
        #[arg(short, long)]
        year: Option<i32>,
        /// Month (1-12), defaults to current month
        #[arg(short, long)]
        month: Option<u32>,
    },
}

fn parse_time_duration(s: &str) -> crate::error::Result<i64> {
    let s = s.trim().to_lowercase();
    if let Some(mins) = s.strip_suffix('m').or_else(|| s.strip_suffix("min")) {
        if let Ok(m) = mins.trim().parse::<f64>() {
            return Ok((m * 60.0) as i64);
        }
    }
    if let Some(hrs) = s
        .strip_suffix('h')
        .or_else(|| s.strip_suffix("hr"))
        .or_else(|| s.strip_suffix("hour"))
    {
        if let Ok(h) = hrs.trim().parse::<f64>() {
            return Ok((h * 3600.0) as i64);
        }
    }
    let mut total_seconds: i64 = 0;
    let mut found_any = false;
    for part in s.split_whitespace() {
        if part.ends_with('h') || part.ends_with("hr") {
            let num = part.trim_end_matches("hr").trim_end_matches('h');
            if let Ok(h) = num.parse::<f64>() {
                total_seconds += (h * 3600.0) as i64;
                found_any = true;
            }
        } else if part.ends_with('m') || part.ends_with("min") {
            let num = part.trim_end_matches("min").trim_end_matches('m');
            if let Ok(m) = num.parse::<f64>() {
                total_seconds += (m * 60.0) as i64;
                found_any = true;
            }
        }
    }
    if found_any {
        return Ok(total_seconds);
    }
    if let Ok(n) = s.parse::<f64>() {
        return Ok((n * 60.0) as i64);
    }
    Err(crate::error::DevBoardError::InvalidInput(format!(
        "Cannot parse time: \"{}\". Examples: 2h 30m, 90m, 1.5h",
        s
    )))
}

pub fn handle(args: TimeCommands) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    match args {
        TimeCommands::Log {
            task,
            time,
            note,
            actor,
        } => match parse_time_duration(&time) {
            Ok(seconds) => {
                let actor_val = actor.as_deref().unwrap_or("human");
                match time_entry::create_time_entry(
                    &conn,
                    task,
                    actor_val,
                    seconds,
                    note.as_deref(),
                ) {
                    Ok(entry) => {
                        let hours = entry.duration as f64 / 3600.0;
                        println!("Logged {:.1}h to task {} (id: {})", hours, task, entry.id);
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        TimeCommands::Report { date } => {
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            let target_date = date.as_deref().unwrap_or(&today);
            let date_naive = match chrono::NaiveDate::parse_from_str(target_date, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => {
                    eprintln!("Invalid date format: {}", target_date);
                    return;
                }
            };
            println!("=== Time Report for {} ===\n", target_date);
            match time_entry::get_daily_time_summary(&conn, date_naive) {
                Ok(stats) => {
                    if stats.is_empty() {
                        println!("No time entries for this date.");
                    } else {
                        println!("{:<10} {:>10}", "Actor", "Hours");
                        println!("{}", "-".repeat(22));
                        let mut total = 0i64;
                        for (actor, secs) in &stats {
                            println!("{:<10} {:>10.1}h", actor, *secs as f64 / 3600.0);
                            total += secs;
                        }
                        println!("{}", "-".repeat(22));
                        println!("{:<10} {:>10.1}h", "Total", total as f64 / 3600.0);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        TimeCommands::Weekly { date } => {
            handle_weekly_report(date);
        }
        TimeCommands::Monthly { year, month } => {
            handle_monthly_report(year, month);
        }
    }
}

pub fn handle_weekly_report(start_date: Option<String>) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let target_date = start_date.as_deref().unwrap_or(&today);
    let date_naive = match chrono::NaiveDate::parse_from_str(target_date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Invalid date format: {}", target_date);
            return;
        }
    };

    println!("=== Weekly Report (starting {}) ===\n", target_date);
    match time_entry::get_weekly_summary(&conn, date_naive) {
        Ok(stats) => {
            if stats.is_empty() {
                println!("No time entries for this week.");
            } else {
                println!("{:<10} {:>10}", "Actor", "Hours");
                println!("{}", "-".repeat(22));
                let mut total = 0i64;
                for (actor, secs) in &stats {
                    println!("{:<10} {:>10.1}h", actor, *secs as f64 / 3600.0);
                    total += secs;
                }
                println!("{}", "-".repeat(22));
                println!("{:<10} {:>10.1}h", "Total", total as f64 / 3600.0);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn handle_monthly_report(year: Option<i32>, month: Option<u32>) {
    let (conn, _) = match get_db() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let now = chrono::Local::now();
    let target_year = year.unwrap_or(now.format("%Y").to_string().parse().unwrap());
    let target_month = month.unwrap_or(now.format("%m").to_string().parse().unwrap());

    println!(
        "=== Monthly Report for {}-{:02} ===\n",
        target_year, target_month
    );
    match time_entry::get_monthly_summary(&conn, target_year, target_month) {
        Ok(stats) => {
            if stats.is_empty() {
                println!("No time entries for this month.");
            } else {
                println!("{:<10} {:>10}", "Actor", "Hours");
                println!("{}", "-".repeat(22));
                let mut total = 0i64;
                for (actor, secs) in &stats {
                    println!("{:<10} {:>10.1}h", actor, *secs as f64 / 3600.0);
                    total += secs;
                }
                println!("{}", "-".repeat(22));
                println!("{:<10} {:>10.1}h", "Total", total as f64 / 3600.0);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
