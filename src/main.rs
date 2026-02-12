use clap::{Parser, ValueEnum};
use moodle_course_builder::Pipeline;
use rand::Rng;
use rand::rngs::OsRng;
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
    #[arg(long, default_value = "2025-08-01")]
    start_date: String,

    /// End date (YYYY-MM-DD)
    #[arg(long, default_value = "2026-06-01")]
    end_date: String,

    /// Department name
    #[arg(long)]
    dept: String,
}

fn get_category_path(
    row: &moodle_course_builder::Record,
    campus: String,
    dept: String,
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
        dept.to_uppercase(),
        program
    )
}

fn get_course_start_date(
    row: &moodle_course_builder::Record,
    start_date: String,
    end_date: String,
) -> String {
    let semester = row.get("Semester").map(|s| s.as_str()).unwrap_or("");
    match semester {
        "1" => format!("{}-08-01", &start_date[0..4]),
        "2" => format!("{}-01-20", &end_date[0..4]),
        &_ => "".to_string(),
    }
}

fn get_course_end_date(
    row: &moodle_course_builder::Record,
    end_date: String,
    start_date: String,
) -> String {
    let semester = row.get("Semester").map(|s| s.as_str()).unwrap_or("");
    match semester {
        "1" => format!("{}-12-18", &start_date[0..4]),
        "2" => format!("{}-06-01", &end_date[0..4]),
        &_ => "".to_string(),
    }
}

fn get_short_name(
    row: &moodle_course_builder::Record,
    campus: String,
    start_date: String,
    end_date: String,
) -> String {
    let semester = row.get("Semester").map(|s| s.as_str()).unwrap_or("");
    let start_year = &start_date[2..4];
    let end_year = &end_date[2..4];
    let semester_tag = format!("S{}{}{}", semester, start_year, end_year);
    let course_code = row
        .get("Course Code")
        .map(|s| s.as_str())
        .unwrap_or("")
        .replace(' ', "");
    let course_code_with_edp = format!("{}-{}", course_code, generate_random_edp_code());
    format!(
        "{}-{}-{}",
        campus.to_uppercase(),
        semester_tag,
        course_code_with_edp
    )
}

pub fn generate_random_edp_code() -> String {
    let mut rng = OsRng;

    let number: u32 = rng.gen_range(0..100_000); // 0 to 99999 inclusive
    format!("{:05}", number) // pads with leading zeros
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
            get_short_name(
                row,
                cli.campus.to_string(),
                start_date.clone(),
                end_date.clone(),
            )
        })
        .derive("fullname", |row| {
            row.get("Descriptive Title").cloned().unwrap_or_default()
        })
        .derive("category_path", |row| {
            get_category_path(
                row,
                cli.campus.to_string(),
                cli.dept.to_string(),
                start_date.clone(),
                end_date.clone(),
            )
        })
        .derive("startdate", |row| {
            get_course_start_date(row, start_date.clone(), end_date.clone())
        })
        .derive("enddate", |row| {
            get_course_end_date(row, end_date.clone(), start_date.clone())
        })
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
