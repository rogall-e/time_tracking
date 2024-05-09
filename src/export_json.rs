use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions};
use std::io::{Write, Result};

#[derive(Serialize, Deserialize)]
pub struct Worktime {
    pub date: String,
    pub starttime: String,
    pub endtime: String,
}

impl Worktime {
    pub fn new(date: String, starttime: String, endtime: String) -> Self {
        Worktime {
            date,
            starttime,
            endtime,
        }
    }

    fn to_jsonl(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn export_json(&mut self) -> Result<()> {
        let filename = "worktime.jsonl";
        if std::path::Path::new(filename).exists() {
            append_worktime_to_jsonl(self, filename)?;
        } else {
            create_and_write_jsonl(self, filename)?;
        }
        Ok(())
    }
}

fn append_worktime_to_jsonl(worktime: &Worktime, filename: &str) -> Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(filename)?;
    writeln!(&file, "{}", worktime.to_jsonl())?;
    Ok(())
}

fn create_and_write_jsonl(worktime: &Worktime, filename: &str) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)?;
    writeln!(&file, "{}", worktime.to_jsonl())?;
    Ok(())
}