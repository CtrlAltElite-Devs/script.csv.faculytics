use clap::{Parser, ValueEnum};
use moodle_course_builder::Pipeline;
use std::error::Error;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Campus {
    Ucmn,
    Uclm,
    Ucb,
    Ucmetc,
    Ucpt,
}

impl std::fmt::Display for Campus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Campus::Ucmn => "ucmn",
            Campus::Uclm => "uclm",
            Campus::Ucb => "ucb",
            Campus::Ucmetc => "ucmetc",
            Campus::Ucpt => "ucpt",
        };
        write!(f, "{}", s)
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the input CSV file
    #[arg(long)]
    in_path: String,

    /// Path to the output CSV file
    #[arg(long)]
    out_path: String,

    /// Campus identifier
    #[arg(long, value_enum)]
    campus: Campus,

    /// Start date (YYYY-MM-DD)
    #[arg(long)]
    start_date: String,

    /// End date (YYYY-MM-DD)
    #[arg(long)]
    end_date: String,

    /// Department name
    #[arg(long)]
    dept: String,
}

fn get_category_path(
    row: &moodle_course_builder::Record,
    campus: String,
    start_date: String,
    end_date: String,
) -> String {
    let program = row.get("Program").map(|s| s.as_str()).unwrap_or("GEN");
    let semester = row.get("Semester").map(|s| s.as_str()).unwrap_or("");
    let start_year = &start_date[2..4];
    let end_year = &end_date[2..4];
    let semester_tag = format!("S{}{}{}", semester, start_year, end_year);
    format!(
        "{} / {} / {} / {}",
        campus.to_uppercase(),
        semester_tag,
        campus.to_uppercase(),
        program
    )
}

fn get_course_date(row: &moodle_course_builder::Record, date: String) -> String {
    let semester = row.get("Semester").map(|s| s.as_str()).unwrap_or("");
    match semester {
        "1" => format!("{}-08-01", &date[0..4]),
        "2" => format!("{}-06-01", &date[0..4]),
        &_ => "".to_string(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Starting transformation pipeline...");
    println!("Input: {}", cli.in_path);
    println!("Output: {}", cli.out_path);

    let start_date = cli.start_date;
    let end_date = cli.end_date;

    Pipeline::new()
        .from_file(&cli.in_path)?
        .derive("shortname", |row| {
            row.get("Course Code").cloned().unwrap_or_default()
        })
        .derive("fullname", |row| {
            row.get("Descriptive Title").cloned().unwrap_or_default()
        })
        .derive("category_path", |row| {
            get_category_path(
                row,
                cli.campus.to_string(),
                start_date.clone(),
                end_date.clone(),
            )
        })
        .derive("startdate", |row| get_course_date(row, start_date.clone()))
        .derive("enddate", |row| get_course_date(row, end_date.clone()))
        .derive("visible", |_| "1".to_string())
        .select(vec![
            "shortname",
            "fullname",
            "category_path",
            "startdate",
            "enddate",
            "visible",
        ])
        .to_file(
            &cli.out_path,
            vec![
                "shortname",
                "fullname",
                "category_path",
                "startdate",
                "enddate",
                "visible",
            ],
        )?;

    println!("Transformation complete.");

    Ok(())
}
