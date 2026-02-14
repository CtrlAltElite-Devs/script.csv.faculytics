use crate::campus::Campus;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum CommandMode {
    Course,
    UserSeed,
    UserEnrol,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Mapping mode
    #[arg(long, value_enum, default_value_t = CommandMode::Course)]
    pub mode: CommandMode,

    // Courses where the users will be enrolled
    #[arg(long, num_args = 1.., required_if_eq("mode", "user-enrol"))]
    pub courses: Vec<String>,

    /// Path to the input CSV file
    #[arg(long, required_if_eq("mode", "course"))]
    pub in_path: Option<String>,

    /// Path to the output CSV file
    #[arg(long)]
    pub out_path: String,

    /// Campus identifier
    #[arg(long, value_enum)]
    pub campus: Campus,

    /// Start date (YYYY-MM-DD)
    #[arg(long, default_value = "2025-08-01")]
    pub start_date: String,

    /// End date (YYYY-MM-DD)
    #[arg(long, default_value = "2026-06-01")]
    pub end_date: String,

    /// Department name
    #[arg(long)]
    pub dept: Option<String>,

    /// Number of users to generate
    #[arg(long, default_value = "1")]
    pub count: usize,

    /// Run transformations without writing output
    #[arg(long)]
    pub dry_run: bool,

    /// Show the first N output records in dry run mode
    #[arg(long, requires = "dry_run")]
    pub dry_run_show: Option<usize>,
}
