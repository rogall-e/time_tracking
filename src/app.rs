use crate::export_json::Worktime;
use anyhow::Result;
use chrono::Local;
use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    EditingStarttime,
    EditingEndtime,
    Exiting,
}

pub enum CurrentlyEditing {
    Starttime,
    Endtime,
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
        }
    }

    // --snip--
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

    pub fn export_json(&self) -> Result<()> {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let mut worktime = Worktime::new(
            date,
            self.starttime_pairs
                .get(&self.starttime_key)
                .unwrap()
                .clone(),
            self.endtime_pairs.get(&self.endtime_key).unwrap().clone(),
        );

        worktime.export_json()?;
        Ok(())
    }
    // --snip--
}
