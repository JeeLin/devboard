use clap::Subcommand;
use std::path::PathBuf;

fn get_notes_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".devboard")
        .join("notes")
}

#[derive(Subcommand)]
pub enum NoteArgs {
    /// Add a quick note
    Add {
        /// Note content
        content: String,
    },
    /// List notes for a date
    List {
        /// Date (YYYY-MM-DD), defaults to today
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Show today's notes
    Today,
}

pub fn handle(args: NoteArgs) {
    match args {
        NoteArgs::Add { content } => {
            let notes_dir = get_notes_dir();
            std::fs::create_dir_all(&notes_dir).unwrap_or_else(|e| {
                eprintln!("Error creating notes directory: {}", e);
                std::process::exit(1);
            });

            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            let note_file = notes_dir.join(format!("{}.md", today));

            let timestamp = chrono::Local::now().format("%H:%M").to_string();
            let entry = format!("\n- [{}] {}\n", timestamp, content);

            use std::io::Write;
            let mut file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&note_file)
                .unwrap_or_else(|e| {
                    eprintln!("Error opening note file: {}", e);
                    std::process::exit(1);
                });

            file.write_all(entry.as_bytes()).unwrap_or_else(|e| {
                eprintln!("Error writing note: {}", e);
                std::process::exit(1);
            });

            println!("Note added: {}", note_file.display());
        }
        NoteArgs::List { date } => {
            let notes_dir = get_notes_dir();
            let target_date = date.unwrap_or_else(|| {
                chrono::Local::now().format("%Y-%m-%d").to_string()
            });
            let note_file = notes_dir.join(format!("{}.md", target_date));

            if !note_file.exists() {
                println!("No notes for {}", target_date);
                return;
            }

            match std::fs::read_to_string(&note_file) {
                Ok(content) => {
                    println!("=== Notes for {} ===\n", target_date);
                    print!("{}", content);
                }
                Err(e) => eprintln!("Error reading notes: {}", e),
            }
        }
        NoteArgs::Today => {
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            let notes_dir = get_notes_dir();
            let note_file = notes_dir.join(format!("{}.md", today));

            if !note_file.exists() {
                println!("No notes today.");
                return;
            }

            match std::fs::read_to_string(&note_file) {
                Ok(content) => {
                    println!("=== Today's Notes ===\n");
                    print!("{}", content);
                }
                Err(e) => eprintln!("Error reading notes: {}", e),
            }
        }
    }
}
