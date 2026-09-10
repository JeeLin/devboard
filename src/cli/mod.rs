pub mod milestone;
pub mod project;
pub mod task;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "devboard",
    version,
    about = "AI-driven project management tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage projects
    #[command(subcommand)]
    Project(project::ProjectArgs),
    /// Manage milestones
    #[command(subcommand)]
    Milestone(milestone::MilestoneArgs),
    /// Manage tasks
    #[command(subcommand)]
    Task(task::TaskArgs),
}
