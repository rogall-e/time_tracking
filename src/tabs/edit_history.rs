use ratatui::{
    buffer::Buffer, 
    layout::{
        Constraint,  
        Layout, 
        Rect
    }, 
    style::{
        palette::tailwind, Color,  Style, 
    }, 
    symbols::border::PROPORTIONAL_TALL, 
    widgets::{
        Block, List, ListItem, ListState, Padding, Paragraph, Widget, StatefulWidget
    }
};
use crate::read_json::{read_json, Worktime};
use unicode_width::UnicodeWidthStr;
use itertools::Itertools;

const NORMAL_ROW_BG: Color = tailwind::SLATE.c950;
const ALT_ROW_BG_COLOR: Color = tailwind::SLATE.c900;



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EditHistoryTab {
    row_index: usize,
}


impl EditHistoryTab {
    pub fn new() -> Self {
        Self {
            row_index: 0,
        }
    }
    pub fn previous(&mut self) {
        self.row_index = self.row_index.saturating_sub(1);
    }
    pub fn next(&mut self) {
        self.row_index = self.row_index.saturating_add(1);
    }

    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(tailwind::GREEN.c700)
    }
}

impl Widget for EditHistoryTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.block().render(area, buf);
        let vertical = Layout::vertical([Constraint::Length(5), Constraint::Min(0)]);
        let [overview, day] = vertical.areas(area);
        render_overview(self.row_index, overview, buf);
        render_day(self.row_index, day, buf);
    }
}
fn render_overview(row_index: usize, area: Rect, buf: &mut Buffer) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let [tabs, _overview] = vertical.areas(area);

    let highlight_symbol = ">>";

    let worktime = match read_json() {
        Ok(worktime) => {
            let mut worktime = worktime;
            worktime.sort_by(|a, b| a.date.cmp(&b.date));
            worktime
        }
        Err(_e) => {
            vec![]
        }
    };

    let date_width = worktime
        .iter()
        .map(|e| e.date.width())
        .max()
        .unwrap_or_default();

    let starttime_width = worktime
        .iter()
        .map(|e| e.starttime.width())
        .max()
        .unwrap_or_default();

    let endtime_width = worktime
        .iter()
        .map(|e| e.endtime.width())
        .max()
        .unwrap_or_default();

    let items = worktime
        .iter()
        .map(|e| {
            let date = format!("{:width$}", e.date, width = date_width).into();
            let starttime = format!("{:width$}", e.starttime, width = starttime_width).into();
            let endtime = format!("{:width$}", e.endtime, width = endtime_width).into();
            ListItem::new(vec![date, starttime, endtime])
            }
        )
        .collect_vec();

    let mut state = ListState::default().with_selected(Some(row_index));
    StatefulWidget::render(
        List::new(items)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol(highlight_symbol),

        tabs,
        buf,
        &mut state,
    )
        
}

fn render_day(row_index: usize, area: Rect, buf: &mut Buffer) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
    let [_tabs, day] = vertical.areas(area);

    let worktime = match read_json() {
        Ok(worktime) => {
            let mut worktime = worktime;
            worktime.sort_by(|a, b| a.date.cmp(&b.date));
            worktime
        }
        Err(_e) => {
            vec![]
        }
    };

    let worktime = worktime.get(row_index).unwrap();

    let date = format!("Date: {}", worktime.date);
    let starttime = format!("Starttime: {}", worktime.starttime);
    let endtime = format!("Endtime: {}", worktime.endtime);

    let date = Paragraph::new(date);
    let starttime = Paragraph::new(starttime);
    let endtime = Paragraph::new(endtime);
    
        
    let meeting_name_width = worktime
        .meetings
        .iter()
        .map(|e| e.meeting_name.width())
        .max()
        .unwrap_or_default();

    let meeting_start_time_width = worktime
        .meetings
        .iter()
        .map(|e| e.meeting_start_time.width())
        .max()
        .unwrap_or_default();

    let meeting_end_time_width = worktime
        .meetings
        .iter()
        .map(|e| e.meeting_end_time.width())
        .max()
        .unwrap_or_default();

    let meetings_items = worktime
        .meetings
        .iter()
        .map(|e| {
            let meeting_name = format!("{:width$}", e.meeting_name, width = meeting_name_width).into();
            let meeting_start_time = format!("{:width$}", e.meeting_start_time, width = meeting_start_time_width).into();
            let meeting_end_time = format!("{:width$}", e.meeting_end_time, width = meeting_end_time_width).into();
            ListItem::new(vec![meeting_name, meeting_start_time, meeting_end_time])
            }
        )
        .collect_vec();

    let meetings_list = List::new(meetings_items);

    // Focus time list
    let focus_time_start_width = worktime
        .focus_time
        .iter()
        .map(|e| e.focus_time_start.width())
        .max()
        .unwrap_or_default();

    let focus_time_end_width = worktime
        .focus_time
        .iter()
        .map(|e| e.focus_time_end.width())
        .max()
        .unwrap_or_default();

    let focus_time_items = worktime
        .focus_time
        .iter()
        .map(|e| {
            let focus_time_start = format!("{:width$}", e.focus_time_start, width = focus_time_start_width).into();
            let focus_time_end = format!("{:width$}", e.focus_time_end, width = focus_time_end_width).into();
            ListItem::new(vec![focus_time_start, focus_time_end])
            }
        )
        .collect_vec();

    let focus_time_list = List::new(focus_time_items);

    let horizontal = Layout::horizontal([Constraint::Min(0); 3]);
    let [date_area, meetings_area, focus_time_area] = horizontal.areas(day);
    let vertical = Layout::vertical([Constraint::Length(1); 3]);
    let [date_area, starttime_area, endtime_area] = vertical.areas(date_area);

    date.render(date_area, buf);
    starttime.render(starttime_area, buf);
    endtime.render(endtime_area, buf);
    Widget::render(meetings_list, meetings_area, buf);
    Widget::render(focus_time_list, focus_time_area, buf);
}     


const fn alternate_colors_list(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}