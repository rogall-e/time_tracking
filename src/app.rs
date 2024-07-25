use crate::{export_json::{FocusTime, MeetingList, Worktime}, read_json::read_json};
use anyhow::Result;
use chrono::Local;
use std::collections::HashMap;
use std::io::prelude::*;
use ratatui::{
    buffer::Buffer,
    widgets::{ScrollbarState, Tabs, Widget, ListState},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind, Color,
    },
};
use crate::tabs::{BarChartTab, EditHistoryTab, FocusTimeTab, MeetingNotesTab};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

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

#[derive(Debug, Clone, Copy, Default, Display, EnumIter, FromRepr, PartialEq, Eq)]
pub enum Tab {
    #[default]
    BarChartTab,
    EditHistoryTab,
    FocusTimeTab,
    MeetingNotesTab,
}

impl Tab {
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    fn prev(self) -> Self {
        let current_index = self as usize;
        let prev_index = current_index.saturating_sub(1);
        Self::from_repr(prev_index).unwrap_or(self)
    }

    fn title(self) -> String {
        match self {
            Self::BarChartTab => String::new(),
            tab => format!(" {tab} "),
        }
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::BarChartTab => tailwind::INDIGO,
            Self::EditHistoryTab => tailwind::EMERALD,
            Self::FocusTimeTab => tailwind::BLUE,
            Self::MeetingNotesTab => tailwind::GREEN,
        }
    }  
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
    pub list_state: ListState,
    pub last_selected: Option<usize>,
    pub focus: bool,
    pub focus_time: u64,
    pub focus_time_list: Vec<FocusTime>,
    pub focus_time_start: String,
    pub focus_time_end: String,
    pub focus_time_total: u64,
    pub tab: Tab,
    pub focus_time_tab: FocusTimeTab,
    pub meeting_notes_tab: MeetingNotesTab,
    pub edit_history_tab: EditHistoryTab,
    pub barchart_tab: BarChartTab,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0)]);
        let [header_area, inner_area] = vertical.areas(area);

        self.render_tabs(header_area, buf);
        self.render_selected_tab(inner_area, buf);
        
    }
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
            list_state: ListState::default(),
            last_selected: None,
            focus: false,
            focus_time: 0,
            focus_time_list: Vec::new(),
            focus_time_start: String::new(),
            focus_time_end: String::new(),
            focus_time_total: 0,
            tab: Tab::BarChartTab,
            focus_time_tab: FocusTimeTab::new(),
            meeting_notes_tab: MeetingNotesTab::new(),
            edit_history_tab: EditHistoryTab::new(),
            barchart_tab: BarChartTab::new(),
        }
    }

    pub fn next_tab(&mut self) {
        self.tab = self.tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.tab = self.tab.prev();
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = Tab::iter().map(Tab::title);
        let highlight_style = (Color::default(), self.tab.palette().c700);
        let selected_tab_index = self.tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }

    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        match self.tab {
            Tab::BarChartTab => self.barchart_tab.clone().render(area, buf),
            Tab::EditHistoryTab => self.edit_history_tab.render(area, buf),
            Tab::FocusTimeTab => self.focus_time_tab.render(area, buf),
            Tab::MeetingNotesTab => self.meeting_notes_tab.clone().render(area, buf),
        };
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

    pub fn start_meeting(&mut self) {
        self.meeting_running = true;
        self.meeting_name = self.meeting_name_input.clone();
        self.meeting_name_input = String::new();
        self.meeting_start_time = Local::now().format("%H:%M").to_string();
        self.currently_editing = None;
        self.tab = Tab::MeetingNotesTab;
        //self.time_in_meetings = meeting_timer(self.meeting_running);
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

    pub fn previous_list_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.get_data_len()  - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.list_state.select(Some(i));
    }

    pub fn next_list_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.get_data_len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.list_state.select(Some(i));
    }

    pub fn get_data_len(&self) -> usize {
        match read_json() {
            Ok(json_response) => {
                json_response.len()
            }
            Err(_) => {
                0
            }
        }
    }

    pub fn start_focus_time(&mut self) {
        self.focus = true;
        self.focus_time_start = Local::now().format("%H:%M").to_string();
    }

    pub fn end_focus_time(&mut self) {
        self.focus = false;
        self.focus_time_end = Local::now().format("%H:%M").to_string();
        let focus_time = FocusTime {
            focus_time_start: self.focus_time_start.clone(),
            focus_time_end: self.focus_time_end.clone(),
            focus_time: self.focus_time as i32,
        };
        self.focus_time_list.push(focus_time);
        self.focus_time_total += self.focus_time;
        self.focus_time = 0;
    }

    pub fn chache_focus_time(&mut self) {
        let mut focus_cache_file = std::fs::File::create(".tmp_cache/focus_cache.bin").unwrap();
        let export_focus: String = self.focus.to_string() + &','.to_string() + &self.focus_time.to_string();
        focus_cache_file.write_all(export_focus.as_bytes()).unwrap();
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
            self.focus_time_list.clone(),
        );

        worktime.export_json()?;
        Ok(())
    }
}
