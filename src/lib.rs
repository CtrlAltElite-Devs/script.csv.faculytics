use csv::{ReaderBuilder, WriterBuilder};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub type Record = HashMap<String, String>;

pub struct Pipeline {
    records: Vec<Record>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn from_file(mut self, path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut rdr = ReaderBuilder::new().from_reader(file);
        
        let headers = rdr.headers()?.clone();
        let mut records = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let mut map = HashMap::new();
            for (header, value) in headers.iter().zip(record.iter()) {
                map.insert(header.to_string(), value.to_string());
            }
            records.push(map);
        }

        self.records = records;
        Ok(self)
    }

    pub fn derive<F>(mut self, header: &str, transform: F) -> Self
    where
        F: Fn(&Record) -> String,
    {
        for record in &mut self.records {
            let new_value = transform(record);
            record.insert(header.to_string(), new_value);
        }
        self
    }

    pub fn select(mut self, headers: Vec<&str>) -> Self {
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
        }
        self
    }

    pub fn to_file(self, path: &str, headers: Vec<&str>) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let mut wtr = WriterBuilder::new().from_writer(file);

        wtr.write_record(&headers)?;

        for record in self.records {
            let mut row = Vec::new();
            for &header in &headers {
                row.push(record.get(header).cloned().unwrap_or_default());
            }
            wtr.write_record(&row)?;
        }

        wtr.flush()?;
        Ok(())
    }
}
