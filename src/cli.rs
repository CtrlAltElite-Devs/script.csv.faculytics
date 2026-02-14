use clap::Parser;
use crate::campus::Campus;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the input CSV file
    #[arg(long)]
    pub in_path: String,

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
    pub dept: String,
}
