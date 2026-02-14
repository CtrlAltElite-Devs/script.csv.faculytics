use crate::constants::*;
use moodle_course_builder::Record;

pub fn print_preview_table(records: &[Record], headers: &[&str], limit: usize) {
    if records.is_empty() || limit == 0 {
        println!("No records to preview.");
        return;
    }

    let take = limit.min(records.len());
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for record in records.iter().take(take) {
        for (idx, &header) in headers.iter().enumerate() {
            let value = record.get(header).map(|s| s.as_str()).unwrap_or("");
            if value.len() > widths[idx] {
                widths[idx] = value.len();
            }
        }
    }

    let mut header_row = String::new();
    for (idx, &header) in headers.iter().enumerate() {
        header_row.push_str(&format!("| {:width$} ", header, width = widths[idx]));
    }
    header_row.push('|');
    println!("{}", header_row);

    let mut separator = String::new();
    for width in &widths {
        separator.push_str(&format!("+{}-", "-".repeat(*width + 2)));
    }
    separator.push('+');
    println!("{}", separator);

    for record in records.iter().take(take) {
        let mut row = String::new();
        for (idx, &header) in headers.iter().enumerate() {
            let value = record.get(header).map(|s| s.as_str()).unwrap_or("");
            row.push_str(&format!("| {:width$} ", value, width = widths[idx]));
        }
        row.push('|');
        println!("{}", row);
    }
}

pub fn report_missing(records: &[Record], headers: &[&str]) {
    if records.is_empty() {
        println!("No records to check for missing values.");
        return;
    }

    let mut missing_counts = vec![0usize; headers.len()];
    let mut missing_rows: Vec<(usize, Vec<&str>, Option<String>, Option<String>)> = Vec::new();

    for (idx, record) in records.iter().enumerate() {
        let mut missing_headers = Vec::new();
        for (h_idx, &header) in headers.iter().enumerate() {
            let value = record.get(header).map(|s| s.trim()).unwrap_or("");
            if value.is_empty() {
                missing_counts[h_idx] += 1;
                missing_headers.push(header);
            }
        }

        if !missing_headers.is_empty() {
            let shortname = record.get(HEADER_SHORTNAME).cloned();
            let fullname = record.get(HEADER_FULLNAME).cloned();
            missing_rows.push((idx + 1, missing_headers, shortname, fullname));
        }
    }

    println!("Missing values summary (output headers):");
    for (idx, &header) in headers.iter().enumerate() {
        println!("- {}: {}", header, missing_counts[idx]);
    }

    if missing_rows.is_empty() {
        println!("No records with missing output values.");
        return;
    }

    println!("Records with missing values:");
    for (row_idx, missing_headers, shortname, fullname) in missing_rows {
        let mut details = format!("Row {}: missing [{}]", row_idx, missing_headers.join(", "));
        if let Some(value) = shortname {
            if !value.trim().is_empty() {
                details.push_str(&format!(" shortname={}", value));
            }
        }
        if let Some(value) = fullname {
            if !value.trim().is_empty() {
                details.push_str(&format!(" fullname={}", value));
            }
        }
        println!("{}", details);
    }
}
