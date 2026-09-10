mod cli;
mod db;
mod error;
mod models;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Project(args) => cli::project::handle(args),
        Commands::Milestone(args) => cli::milestone::handle(args),
        Commands::Task(args) => cli::task::handle(args),
    }
}
