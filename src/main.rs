mod cli;
mod db;
mod error;
mod models;
mod tui;
#[allow(dead_code)]
mod charts;
mod sync;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Project(args) => cli::project::handle(args),
        Commands::Milestone(args) => cli::milestone::handle(args),
        Commands::Task(args) => cli::task::handle(args),
        Commands::Time(args) => cli::time::handle(args),
        Commands::Doc(args) => cli::doc::handle(args),
        Commands::Search(args) => cli::search::handle(args),
        Commands::Tag(args) => cli::tag::handle(args),
        Commands::Note(args) => cli::note::handle(args),
        Commands::Template(args) => cli::template::handle(args),
        Commands::Chart(args) => cli::chart::handle(args),
        Commands::Sync(args) => cli::sync::handle(args),
    }
}
