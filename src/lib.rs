use csv::{ReaderBuilder, WriterBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub type Record = HashMap<String, String>;

pub struct Pipeline {
    records: Vec<Record>,
}

fn default_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{msg:>30.cyan.bold} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
    )
    .unwrap()
    .progress_chars("=>-")
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn from_records(records: Vec<Record>) -> Self {
        Self { records }
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Record) -> bool,
    {
        let pb = ProgressBar::new(self.records.len() as u64);
        pb.set_style(default_style());
        pb.set_message("Filtering records...");

        let initial_count = self.records.len();
        self.records.retain(|record| {
            let keep = predicate(record);
            pb.inc(1);
            keep
        });

        let filtered_count = self.records.len();
        pb.finish_with_message(format!(
            "Filtered records: {} -> {}",
            initial_count, filtered_count
        ));
        self
    }

    pub fn from_file(mut self, path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg:.cyan.bold}").unwrap(),
        );
        pb.set_message(format!("Reading {}...", path));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));

        let headers = rdr.headers()?.clone();
        let mut records = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let mut map = HashMap::new();
            for (header, value) in headers.iter().zip(record.iter()) {
                map.insert(header.to_string(), value.to_string());
            }
            records.push(map);
            pb.tick();
        }

        pb.finish_with_message(format!("Loaded records from {}", path));
        self.records = records;
        Ok(self)
    }

    pub fn derive<F>(mut self, header: &str, transform: F) -> Self
    where
        F: Fn(&Record) -> String,
    {
        let pb = ProgressBar::new(self.records.len() as u64);
        pb.set_style(default_style());
        pb.set_message(format!("Deriving {}...", header));

        for record in &mut self.records {
            let new_value = transform(record);
            record.insert(header.to_string(), new_value);
            pb.inc(1);
        }

        pb.finish_with_message(format!("Finished deriving {}", header));
        self
    }

    pub fn select(mut self, headers: Vec<&str>) -> Self {
        let pb = ProgressBar::new(self.records.len() as u64);
        pb.set_style(default_style());
        pb.set_message("Selecting headers...");

        for record in &mut self.records {
            let mut new_record = HashMap::new();
            for &header in &headers {
                if let Some(value) = record.get(header) {
                    new_record.insert(header.to_string(), value.clone());
                } else {
                    new_record.insert(header.to_string(), String::new());
                }
            }
            *record = new_record;
            pb.inc(1);
        }

        pb.finish_with_message("Finished header selection");
        self
    }

    pub fn to_file(self, path: &str, headers: Vec<&str>) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file = File::create(path)?;
        let mut wtr = WriterBuilder::new().from_writer(file);

        let mut written_count = 0usize;
        let mut skipped_count = 0usize;
        for record in &self.records {
            let mut complete = true;
            for &header in &headers {
                match record.get(header) {
                    Some(value) if !value.trim().is_empty() => {}
                    _ => {
                        complete = false;
                        break;
                    }
                }
            }
            if complete {
                written_count += 1;
            } else {
                skipped_count += 1;
            }
        }

        let pb = ProgressBar::new(written_count as u64);
        pb.set_style(default_style());
        pb.set_message(format!("Writing to {}...", path));

        wtr.write_record(&headers)?;

        for record in self.records {
            let mut complete = true;
            for &header in &headers {
                match record.get(header) {
                    Some(value) if !value.trim().is_empty() => {}
                    _ => {
                        complete = false;
                        break;
                    }
                }
            }
            if !complete {
                continue;
            }
            let mut row = Vec::new();
            for &header in &headers {
                row.push(record.get(header).cloned().unwrap_or_default());
            }
            wtr.write_record(&row)?;
            pb.inc(1);
        }

        wtr.flush()?;
        if skipped_count > 0 {
            pb.finish_with_message(format!(
                "Finished writing to {} (written {}, skipped {} incomplete)",
                path, written_count, skipped_count
            ));
        } else {
            pb.finish_with_message(format!("Finished writing to {} (written {})", path, written_count));
        }
        Ok(())
    }
}
