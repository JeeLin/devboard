use clap::{Parser, Subcommand};

pub mod chart;
pub mod doc;
pub mod milestone;
pub mod note;
pub mod project;
pub mod search;
pub mod sync;
pub mod tag;
pub mod task;
pub mod template;
pub mod time;

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
    /// Time tracking
    #[command(subcommand)]
    Time(time::TimeCommands),
    /// Manage documents
    #[command(subcommand)]
    Doc(doc::DocArgs),
    /// Full-text search
    Search(search::SearchArgs),
    /// Manage tags
    #[command(subcommand)]
    Tag(tag::TagArgs),
    /// Quick notes
    #[command(subcommand)]
    Note(note::NoteArgs),
    /// Task templates
    #[command(subcommand)]
    Template(template::TemplateArgs),
    /// Charts
    #[command(subcommand)]
    Chart(chart::ChartCommands),
    /// Sync data
    #[command(subcommand)]
    Sync(sync::SyncCommands),
}
