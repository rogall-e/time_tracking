use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
};

use crate::calc_time::parse_time;
use crate::read_json::read_json;

#[derive(Clone)]
pub struct TimeData<'a> {
    pub time: [u64; 6],
    pub label: &'a str, //worktime or meetingtime
    pub bar_style: Style,
}

pub struct BarChartApp<'a> {
    pub data: Vec<TimeData<'a>>,
    pub days: [String; 6],
}

impl<'a> BarChartApp<'a> {
    pub fn new() -> Self {
        match read_json() {
            Ok(worktime_list) => {
                let mut worktime_in_min_list = [0, 0, 0, 0, 0, 0];
                let mut meetingtime_in_min_list = [0, 0, 0, 0, 0, 0];
                let mut days_list:[String; 6] =["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];

                let mut idx = 0;
                for worktime in worktime_list.iter().cloned().rev().take(6).rev() {
                        let date_string = worktime.date;
                        days_list[idx] = date_string;

                        let start_time = parse_time(&worktime.starttime);
                        let end_time = parse_time(&worktime.endtime);
                        let start_minutes = start_time.0 * 60 + start_time.1;
                        let end_minutes = end_time.0 * 60 + end_time.1;
                        let worktime_in_min: i32 = end_minutes - start_minutes;

                        worktime_in_min_list[idx] = worktime_in_min as u64;
    
                        let total_meeting_time:i32 = worktime.meetings
                            .into_iter()
                            .map(|x| x.time_in_meeting)
                            .sum();

                        meetingtime_in_min_list[idx] = total_meeting_time as u64;
                        idx += 1;
                };
                
                BarChartApp {
                    data: [
                        TimeData {
                            time: worktime_in_min_list,
                            label: "Worktime",
                            bar_style: Style::default().fg(Color::Green),
                        },
                        TimeData {
                            time: meetingtime_in_min_list,
                            label: "Meetingtime",
                            bar_style: Style::default().fg(Color::Red),
                        },
                    ].to_vec(),
                    days: days_list,
                }   
                            
            },
            Err(_) => {
                let worktime_in_min_list = [0, 0, 0, 0, 0, 0];
                let meetingtime_in_min_list = [0, 0, 0, 0, 0, 0];
                let days_list:[String; 6] =["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
                BarChartApp {
                    data: [
                        TimeData {
                            time: worktime_in_min_list,
                            label: "Worktime",
                            bar_style: Style::default().fg(Color::Green),
                        },
                        TimeData {
                            time: meetingtime_in_min_list,
                            label: "Meetingtime",
                            bar_style: Style::default().fg(Color::Red),
                        },
                    ].to_vec(),
                    days: days_list,
                }
            },
        }
    }
    pub fn new_current(worktime_in_min: u64, time_in_meetings:u64, date: String) -> Self {
        BarChartApp {
            data: [
                TimeData {
                    time: [worktime_in_min, 0, 0, 0, 0, 0],
                    label: "Worktime",
                    bar_style: Style::default().fg(Color::Green),
                },
                TimeData {
                    time: [time_in_meetings, 0, 0, 0, 0, 0],
                    label: "Meetingtime",
                    bar_style: Style::default().fg(Color::Red),
                },
            ].to_vec(),
            days: [date, "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
        }
    }
}

#[allow(clippy::cast_precision_loss)]
pub fn create_groups<'a>(barchart: &'a BarChartApp) -> Vec<BarGroup<'a>> {
    barchart.days
        .iter()
        .enumerate()
        .map(|(i, days)| {
            let bars: Vec<Bar> = barchart
                .data
                .iter()
                .map(|c| {
                        let mut bar = Bar::default()
                            .value(c.time[i])
                            .style(c.bar_style)
                            .value_style(
                                Style::default()
                                    .bg(c.bar_style.fg.unwrap())
                                    .fg(Color::Black),
                            );
                        
                        bar = bar.text_value(format!(
                            "{}",
                            c.time[i]
                        ));
                        bar
                })
                .collect();
           
            BarGroup::default()
                .label(Line::from(days.as_str()))
                .bars(&bars)
        })
        .collect()
}

#[allow(clippy::cast_possible_truncation)]
pub fn draw_bar_with_group_labels<'a>(barchart: &'a BarChartApp, current_day: bool) -> BarChart<'a>{

    let groups = create_groups(barchart);

    if current_day {
        let mut barchart = BarChart::default()
            .block(Block::bordered().title("Today").style(Style::default().fg(Color::White)))
            .bar_width(5)
            .group_gap(2)
            .bar_gap(0)
            .label_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC),
            )
            .max(700);

        for group in groups {
            barchart = barchart.data(group);
        }
        
        barchart
    } else {
        let mut barchart = BarChart::default()
            .block(Block::bordered().title("Worktime (green) and Time in Meetings (red) per Day").style(Style::default().fg(Color::White)))
            .bar_width(5)
            .group_gap(2)
            .bar_gap(0)
            .label_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC),
            )
            .max(700);

        for group in groups {
            barchart = barchart.data(group);
        }

        barchart
    }
}

