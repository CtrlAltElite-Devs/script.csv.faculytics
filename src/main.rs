use clap::Parser;
use moodle_course_builder::Pipeline;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the input CSV file
    input: String,

    /// Path to the output CSV file
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Starting transformation pipeline...");
    println!("Input: {}", cli.input);
    println!("Output: {}", cli.output);

    Pipeline::new()
        .from_file(&cli.input)?
        .derive("shortname", |row| {
            let course_code = row.get("Course Code").map(|s| s.as_str()).unwrap_or("");
            format!("{}", course_code)
        })
        .derive("fullname", |row| {
            let descriptive_title = row
                .get("Descriptive Title")
                .map(|s| s.as_str())
                .unwrap_or("");
            format!("{}", descriptive_title)
        })
        .derive("category_path", |_row| format!("{}", "todo"))
        .derive("startdate", |_row| format!("{}", "todo"))
        .derive("enddate", |_row| format!("{}", "todo"))
        .derive("visible", |_row| format!("{}", 1))
        .select(vec![
            "shortname",
            "fullname",
            "category_path",
            "startdate",
            "enddate",
            "visible",
        ])
        .to_file(
            &cli.output,
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
