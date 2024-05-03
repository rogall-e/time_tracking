use serde::{Deserialize, Serialize};
use serde_jsonlines::{json_lines, write_json_lines};
use std::fs::File;
use std::io::{BufWriter, Result, Write};

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

    pub fn export_json(&mut self) -> Result<()> {
        let file = File::create("worktime.json")?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self)?;
        writer.flush()?;
        Ok(())
    }
}
