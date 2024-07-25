use serde::{Deserialize, Serialize};
use serde_jsonlines::JsonLinesReader;
use std::fs::File;
use std::io::{BufReader, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingList {
    pub meeting_name: String,
    pub meeting_start_time: String,
    pub meeting_end_time: String,
    pub time_in_meeting: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FocusTime {
    pub focus_time_start: String,
    pub focus_time_end: String,
    pub focus_time: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Worktime {
    pub date: String,
    pub starttime: String,
    pub endtime: String,
    pub meetings: Vec<MeetingList>,
    pub focus_time: Vec<FocusTime>,
}

pub fn read_json() -> Result<Vec<Worktime>> {
    let file = File::open("data/worktime.jsonl")?;
    let reader = BufReader::new(file);
    let json_reader = JsonLinesReader::new(reader);
    let worktime_days = json_reader
        .read_all::<Worktime>()
        .collect::<Result<Vec<_>>>()
        .unwrap();
    Ok(worktime_days)
}

pub async fn get_json_data() -> Vec<Worktime> {
    match read_json() {
        Ok(json_response) => {
            json_response
        }
        Err(_) => {
            let json_response = Vec::<Worktime>::new();
            json_response
        }
    }
}