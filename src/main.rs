mod campus;
mod cli;
mod constants;
mod utils;

use crate::constants::*;
use clap::Parser;
use cli::Cli;
use moodle_course_builder::Pipeline;
use std::error::Error;
use utils::{
    get_category_path, get_course_end_date, get_course_start_date, get_short_name,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Starting transformation pipeline...");
    println!("Input: {}", cli.in_path);
    println!("Output: {}", cli.out_path);

    let campus_upper = cli.campus.to_string().to_uppercase();
    let dept_upper = cli.dept.to_uppercase();
    let start_year_full = &cli.start_date[0..4];
    let end_year_full = &cli.end_date[0..4];
    let start_year_short = &cli.start_date[2..4];
    let end_year_short = &cli.end_date[2..4];

    let output_headers = vec![
        HEADER_SHORTNAME,
        HEADER_FULLNAME,
        HEADER_CATEGORY_PATH,
        HEADER_STARTDATE,
        HEADER_ENDDATE,
        HEADER_VISIBLE,
    ];

    Pipeline::new()
        .from_file(&cli.in_path)?
        .derive(HEADER_SHORTNAME, |row| {
            get_short_name(row, &campus_upper, start_year_short, end_year_short)
        })
        .derive(HEADER_FULLNAME, |row| {
            row.get(HEADER_DESCRIPTIVE_TITLE)
                .cloned()
                .unwrap_or_default()
        })
        .derive(HEADER_CATEGORY_PATH, |row| {
            get_category_path(
                row,
                &campus_upper,
                &dept_upper,
                start_year_short,
                end_year_short,
            )
        })
        .derive(HEADER_STARTDATE, |row| {
            get_course_start_date(row, start_year_full, end_year_full)
        })
        .derive(HEADER_ENDDATE, |row| {
            get_course_end_date(row, start_year_full, end_year_full)
        })
        .derive(HEADER_VISIBLE, |_| VALUE_VISIBLE_TRUE.to_string())
        .select(output_headers.clone())
        .to_file(&cli.out_path, output_headers)?;

    println!("Transformation complete.");

    Ok(())
}
