mod campus;
mod cli;
mod constants;
mod dry_run;
mod utils;

use crate::constants::*;
use clap::Parser;
use cli::{Cli, CommandMode};
use dry_run::{print_preview_table, report_missing};
use moodle_course_builder::{Pipeline, Record};
use std::collections::HashMap;
use std::error::Error;
use utils::{
    generate_fake_user, generate_username, get_category_path, get_course_end_date,
    get_course_start_date, get_short_name,
};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    println!("Starting transformation pipeline in {:?} mode...", cli.mode);

    if let Some(in_path) = cli.in_path.as_deref() {
        println!("Input: {}", in_path);
    }

    println!("Output: {}", cli.out_path);

    match cli.mode {
        CommandMode::Course => run_course_mapping(cli),
        CommandMode::UserSeed => run_user_seeding(cli),
        CommandMode::UserEnrol => run_user_enrolling(cli),
    }
}

fn run_course_mapping(cli: Cli) -> Result<(), Box<dyn Error>> {
    let in_path = cli.in_path.ok_or("cli --in-path not defined")?;

    let campus_upper = cli.campus.to_string().to_uppercase();
    let dept_upper = cli.dept.unwrap_or_default().to_uppercase();
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
        HEADER_OUTPUT_YEAR,
    ];

    let pipeline = Pipeline::new()
        .from_file(&in_path)?
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
        .derive(HEADER_OUTPUT_YEAR, |row| {
            row.get(HEADER_INPUT_YEAR).cloned().unwrap_or_default()
        })
        .select(output_headers.clone());

    if cli.dry_run {
        println!(
            "Dry run: would write {} records to {}",
            pipeline.len(),
            cli.out_path
        );
        if let Some(limit) = cli.dry_run_show {
            println!("Previewing first {} records:", limit);
            print_preview_table(pipeline.records(), &output_headers, limit);
        }
        report_missing(pipeline.records(), &output_headers);
        return Ok(());
    }

    pipeline.to_file(&cli.out_path, output_headers)?;

    println!("Course transformation complete.");
    Ok(())
}

fn run_user_seeding(cli: Cli) -> Result<(), Box<dyn Error>> {
    let mut user_records = Vec::new();
    let campus_str = cli.campus.to_string();

    for _ in 0..cli.count {
        let mut record: Record = HashMap::new();
        let username = generate_username(&campus_str);
        let (firstname, lastname, email) = generate_fake_user();

        record.insert(HEADER_USERNAME.to_string(), username);
        record.insert(HEADER_FIRSTNAME.to_string(), firstname);
        record.insert(HEADER_LASTNAME.to_string(), lastname);
        record.insert(HEADER_EMAIL.to_string(), email);
        record.insert(HEADER_PASSWORD.to_string(), DEFAULT_PASSWORD.to_string());
        user_records.push(record);
    }

    let output_headers = vec![
        HEADER_USERNAME,
        HEADER_FIRSTNAME,
        HEADER_LASTNAME,
        HEADER_EMAIL,
        HEADER_PASSWORD,
    ];

    let all_headers_strings: Vec<String> = output_headers.iter().map(|&s| s.to_string()).collect();
    let all_headers_refs: Vec<&str> = all_headers_strings.iter().map(|s| s.as_str()).collect();

    if cli.dry_run {
        println!(
            "Dry run: would write {} users to {}",
            user_records.len(),
            cli.out_path
        );
        if let Some(limit) = cli.dry_run_show {
            println!("Previewing first {} users:", limit);
            print_preview_table(&user_records, &all_headers_refs, limit);
        }
        return Ok(());
    }

    let pipeline = Pipeline::from_records(user_records);
    pipeline.to_file(&cli.out_path, all_headers_refs)?;

    println!("User generation complete.");
    Ok(())
}

fn run_user_enrolling(cli: Cli) -> Result<(), Box<dyn Error>> {
    let in_path = cli.in_path.ok_or("cli --in-path not defined")?;

    let mut output_headers: Vec<&str> = vec![
        HEADER_USERNAME,
        HEADER_FIRSTNAME,
        HEADER_LASTNAME,
        HEADER_EMAIL,
        HEADER_PASSWORD,
    ];

    let mut dynamic_headers: Vec<String> = Vec::new();
    for (i, _) in cli.courses.iter().enumerate() {
        dynamic_headers.push(format!("course{}", i + 1));
        dynamic_headers.push(format!("role{}", i + 1));
    }
    let dynamic_headers_refs: Vec<&str> = dynamic_headers.iter().map(|s| s.as_str()).collect();
    output_headers.extend(dynamic_headers_refs);

    let mut pipeline = Pipeline::new().from_file(&in_path)?;

    // Transform records to add course and role information
    let transformed_records = pipeline
        .records()
        .iter()
        .map(|record| {
            let mut new_record = record.clone();
            for (i, course_name) in cli.courses.iter().enumerate() {
                new_record.insert(format!("course{}", i + 1), course_name.clone());
                new_record.insert(format!("role{}", i + 1), VALUE_ROLE_STUDENT.to_string());
            }
            new_record
        })
        .collect::<Vec<Record>>();

    pipeline = Pipeline::from_records(transformed_records);
    pipeline = pipeline.select(output_headers.clone());

    if cli.dry_run {
        println!(
            "Dry run: would enroll {} users and write to {}",
            pipeline.len(),
            cli.out_path
        );
        if let Some(limit) = cli.dry_run_show {
            println!("Previewing first {} enrollments:", limit);
            print_preview_table(pipeline.records(), &output_headers, limit);
        }
        return Ok(());
    }

    pipeline.to_file(&cli.out_path, output_headers)?;

    println!("User enrollment complete.");
    Ok(())
}
