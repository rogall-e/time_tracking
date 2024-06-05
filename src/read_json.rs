use serde::{Deserialize, Serialize};
use serde_jsonlines::JsonLinesReader;
use std::fs::File;
use std::io::{BufReader, Result};

#[derive(Serialize, Deserialize)]
pub struct Worktime {
    pub date: String,
    pub starttime: String,
    pub endtime: String,
}

pub fn read_json() -> Result<Vec<Worktime>> {
    let file = File::open("worktime.jsonl")?;
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