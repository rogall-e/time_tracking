use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Result, Write};

#[derive(Clone, Serialize, Deserialize)]
pub struct MeetingList {
    pub meeting_name: String,
    pub meeting_start_time: String,
    pub meeting_end_time: String,
    pub time_in_meeting: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Worktime {
    pub date: String,
    pub starttime: String,
    pub endtime: String,
    pub meetings: Vec<MeetingList>,
}

impl Worktime {
    pub fn new(
        date: String,
        starttime: String,
        endtime: String,
        meetings: Vec<MeetingList>,
    ) -> Self {
        Worktime {
            date,
            starttime,
            endtime,
            meetings,
        }
    }

    fn to_jsonl(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn export_json(&mut self) -> Result<()> {
        let filename = "data/worktime.jsonl";
        if std::path::Path::new(filename).exists() {
            append_worktime_to_jsonl(self, filename)?;
        } else {
            create_and_write_jsonl(self, filename)?;
        }
        Ok(())
    }
}

fn append_worktime_to_jsonl(worktime: &Worktime, filename: &str) -> Result<()> {
    let file = OpenOptions::new().append(true).open(filename)?;
    writeln!(&file, "{}", worktime.to_jsonl())?;
    Ok(())
}

fn create_and_write_jsonl(worktime: &Worktime, filename: &str) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;
    writeln!(&file, "{}", worktime.to_jsonl())?;
    Ok(())
}