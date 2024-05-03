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

pub fn read_json() -> Vec<Worktime> {
    let file = File::open("worktime.json");
    let reader = BufReader::new(file.unwrap());
    let json_reader = JsonLinesReader::new(reader);
    let worktime_days = json_reader
        .read_all::<Worktime>()
        .collect::<Result<Vec<_>>>()
        .unwrap();
    worktime_days
}
