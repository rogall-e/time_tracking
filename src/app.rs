use crate::export_json::{MeetingList, Worktime};
use anyhow::Result;
use chrono::Local;
use std::collections::HashMap;
use ratatui::widgets::ScrollbarState;

pub enum CurrentScreen {
    Main,
    EditingStarttime,
    EditingEndtime,
    EditingMeetingName,
    Exiting,
}

pub enum CurrentlyEditing {
    Starttime,
    Endtime,
    MeetingName,
}

pub struct App {
    pub starttime_key: String,   // the currently being edited json key.
    pub starttime_input: String, // the currently being edited json value.
    pub starttime_pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub endtime_key: String,                      // the currently being edited json key.
    pub endtime_input: String,                    // the currently being edited json value.
    pub endtime_pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub time_in_meetings: i32,
    pub meeting_running: bool,
    pub meeting_start_time: String,
    pub meeting_end_time: String,
    pub meeting_list: Vec<MeetingList>,
    pub meeting_name_input: String,
    pub meeting_name: String,
    pub do_print: bool,
    pub should_exit: bool,
    pub default_starttime: String,
    pub current_worktime: u64,
    pub total_time_in_meetings: i32,
    pub scrollbar_state: ScrollbarState,
    pub horizontal_scroll: usize,
}

impl App {
    pub fn new() -> App {
        App {
            starttime_key: String::from("Starttime"),
            starttime_input: String::new(),
            starttime_pairs: HashMap::new(),
            endtime_key: String::from("Endtime"),
            endtime_input: String::new(),
            endtime_pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            time_in_meetings: 0,
            meeting_running: false,
            meeting_start_time: String::new(),
            meeting_end_time: String::new(),
            meeting_list: Vec::new(),
            meeting_name_input: String::new(),
            meeting_name: String::new(),
            do_print: false,
            should_exit: false,
            default_starttime: String::from("08:45"),
            current_worktime: 0,
            total_time_in_meetings: 0,
            scrollbar_state: ScrollbarState::default(),
            horizontal_scroll: 0,
        }
    }

    pub fn save_starttime_value(&mut self) {
        self.starttime_pairs
            .insert(self.starttime_key.clone(), self.starttime_input.clone());

        self.starttime_input = String::new();

        self.currently_editing = None;
    }

    pub fn save_endtime_value(&mut self) {
        self.endtime_pairs
            .insert(self.endtime_key.clone(), self.endtime_input.clone());

        self.endtime_input = String::new();
        self.currently_editing = None;
    }

    pub fn print_json(&self) -> Result<()> {
        let starttime_output = serde_json::to_string(&self.starttime_pairs)?;
        let endtime_output = serde_json::to_string(&self.endtime_pairs)?;
        println!("{}", starttime_output);
        println!("{}", endtime_output);
        Ok(())
    }

    pub fn save_meeting_name(&mut self) {
        self.meeting_running = true;
        self.meeting_name = self.meeting_name_input.clone();
        self.meeting_name_input = String::new();
        self.meeting_start_time = Local::now().format("%H:%M").to_string();
        self.currently_editing = None;
        //self.time_in_meetings = meeting_timer(self.meeting_running);
    }

    pub fn start_meeting(&mut self) {
        self.meeting_running = true;
        self.meeting_start_time = Local::now().format("%H:%M").to_string();
    }

    pub fn end_meeting(&mut self) {
        self.meeting_running = false;
        self.meeting_end_time = Local::now().format("%H:%M").to_string();
        let meeting = MeetingList {
            meeting_name: self.meeting_name.clone(),
            meeting_start_time: self.meeting_start_time.clone(),
            meeting_end_time: self.meeting_end_time.clone(),
            time_in_meeting: self.time_in_meetings.clone(),
        };
        //self.total_time_in_meetings += self.time_in_meetings;
        self.meeting_list.push(meeting);
        self.time_in_meetings = 0;
    }

    pub fn export_json(&self) -> Result<()> {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let mut worktime = Worktime::new(
            date,
            self.starttime_pairs
                .get(&self.starttime_key)
                .unwrap()
                .clone(),
            self.endtime_pairs.get(&self.endtime_key).unwrap().clone(),
            self.meeting_list.clone(),
        );

        worktime.export_json()?;
        Ok(())
    }
}
