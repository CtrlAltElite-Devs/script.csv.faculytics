use crate::constants::*;
use chrono::Local;
use fake::Fake;
use fake::faker::name::en::{FirstName, LastName};
use moodle_course_builder::Record;
use rand::Rng;
use rand::rngs::OsRng;

pub fn get_category_path(
    row: &Record,
    campus_upper: &str,
    dept_upper: &str,
    start_year_short: &str,
    end_year_short: &str,
) -> String {
    let program = row
        .get(HEADER_PROGRAM)
        .map(|s| s.as_str())
        .unwrap_or(DEFAULT_PROGRAM);
    let semester = row.get(HEADER_SEMESTER).map(|s| s.as_str()).unwrap_or("");
    let semester_tag = format!("S{}{}{}", semester, start_year_short, end_year_short);
    if !KNOWN_SEMESTER_TAGS.contains(&semester_tag.as_str()) {
        return String::new();
    }
    format!(
        "{} / {} / {} / {}",
        campus_upper, semester_tag, dept_upper, program
    )
}

pub fn get_course_start_date(row: &Record, start_year_full: &str, end_year_full: &str) -> String {
    let semester = row.get(HEADER_SEMESTER).map(|s| s.as_str()).unwrap_or("");
    match semester {
        SEMESTER_1 => format!("{}-{}", start_year_full, DATE_SEM1_START),
        SEMESTER_2 => format!("{}-{}", end_year_full, DATE_SEM2_START),
        _ => String::new(),
    }
}

pub fn get_course_end_date(row: &Record, start_year_full: &str, end_year_full: &str) -> String {
    let semester = row.get(HEADER_SEMESTER).map(|s| s.as_str()).unwrap_or("");
    match semester {
        SEMESTER_1 => format!("{}-{}", start_year_full, DATE_SEM1_END),
        SEMESTER_2 => format!("{}-{}", end_year_full, DATE_SEM2_END),
        _ => String::new(),
    }
}

pub fn get_short_name(
    row: &Record,
    campus_upper: &str,
    start_year_short: &str,
    end_year_short: &str,
) -> String {
    let semester = row.get(HEADER_SEMESTER).map(|s| s.as_str()).unwrap_or("");
    let semester_tag = format!("S{}{}{}", semester, start_year_short, end_year_short);
    let course_code = row
        .get(HEADER_COURSE_CODE)
        .map(|s| s.as_str())
        .unwrap_or("")
        .replace(' ', "");
    let course_code_with_edp = format!("{}-{}", course_code, generate_random_edp_code());
    format!("{}-{}-{}", campus_upper, semester_tag, course_code_with_edp)
}

pub fn generate_random_edp_code() -> String {
    let mut rng = OsRng;

    let number: u32 = rng.gen_range(0..100_000); // 0 to 99999 inclusive
    format!("{:05}", number) // pads with leading zeros
}

pub fn generate_username(campus: &str) -> String {
    let now = Local::now();
    let yy = now.format("%y").to_string();
    let m = now
        .format("%m")
        .to_string()
        .trim_start_matches('0')
        .to_string();
    let d = now
        .format("%d")
        .to_string()
        .trim_start_matches('0')
        .to_string();

    let mut rng = OsRng;
    let random_digits: u32 = rng.gen_range(0..10_000);

    format!(
        "{}-{}{}{}{:04}",
        campus.to_lowercase(),
        yy,
        m,
        d,
        random_digits
    )
}

pub fn generate_faculty_username(campus: &str) -> String {
    let mut rng = OsRng;
    let random_digits: u32 = rng.gen_range(0..100_000); // 0 to 99999

    format!("{}-t-{:05}", campus.to_lowercase(), random_digits)
}

pub fn generate_fake_user() -> (String, String, String) {
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let email = format!(
        "{}.{}@email.com",
        first_name.to_lowercase(),
        last_name.to_lowercase()
    );
    (first_name, last_name, email)
}
