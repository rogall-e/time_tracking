use crate::app::{App, CurrentScreen, CurrentlyEditing};
use crate::calc_time::parse_time;
use crate::read_json::read_json;
use crate::transform_digit_to_ascii::{draw_colon, transform_digit_to_ascii};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span, Text},
    widgets::{BarChart, Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use chrono::{Local, NaiveDate};


struct JsonParsed {
    data: Vec<(String, i32)>, // Owned data
}

impl JsonParsed {
    fn new() -> Self {
        match read_json() {
            Ok(worktime_list) => {
                let mut worktime_items = Vec::<(String, i32)>::new(); // Changed the type to own String
                for worktime in worktime_list {
                    let date_str = worktime.date;
                    let start_time = parse_time(&worktime.starttime);
                    let end_time = parse_time(&worktime.endtime);
                    let start_minutes = start_time.0 * 60 + start_time.1;
                    let end_minutes = end_time.0 * 60 + end_time.1;
                    let worktime_in_min: i32 = end_minutes - start_minutes;
                    worktime_items.push((date_str, worktime_in_min));
                }

                JsonParsed {
                    data: worktime_items,
                }
            }
            Err(_) => JsonParsed {
                data: Vec::<(String, i32)>::new(),
            },
        }
    }

    fn data_for_last_seven_days(&self) -> Vec<(&str, u64)> {
        self.data
            .iter()
            .filter(|(date, _)| {
                let today: String = Local::now().format("%Y-%m-%d").to_string();
                let today_date = NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap();
                let worktime_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
                let days_difference = today_date.signed_duration_since(worktime_date).num_days();
                days_difference >= 0 && days_difference <= 7
            })
            .map(|(date, worktime_in_min)| (date.as_str(), *worktime_in_min as u64))
            .collect()
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let inner_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let left_inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_chunks[0]);

    let left_inner_upper_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(3),  
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(2)
            ])
        .split(left_inner_chunks[0]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Time Tracker",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for key in app.starttime_pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.starttime_pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    for key in app.endtime_pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.endtime_pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White)),
    );

    f.render_widget(list, left_inner_chunks[1]);

    let json_data = JsonParsed::new();

    let data: Vec<(&str, u64)> = json_data.data_for_last_seven_days();

    let barchart = BarChart::default()
        .data(&data)
        .block(Block::default().title("Worktime in Minutes").borders(Borders::ALL))
        .bar_width(10)
        .bar_gap(1)
        .value_style(Style::default().fg(Color::White).bg(Color::Green))
        .label_style(Style::default().fg(Color::Yellow)
            .add_modifier(Modifier::ITALIC))
        .style(Style::default().fg(Color::White));

    f.render_widget(barchart, inner_chunks[1]);

    let current_time = Local::now().format("%H:%M").to_string();
    let hour: &str = current_time.split(":").next().unwrap();
    let hour_1: i32 = hour.chars().next().unwrap() as i32 - 0x30;
    let hour_2: i32 = hour.chars().last().unwrap() as i32 - 0x30;
    let minute: &str = current_time.split(":").last().unwrap();
    let minute_1: i32 = minute.chars().next().unwrap() as i32 - 0x30;
    let minute_2: i32 = minute.chars().last().unwrap() as i32 - 0x30;


    let paragraph_1 = Paragraph::new(Text::from(transform_digit_to_ascii(hour_1)));
    let paragraph_2 = Paragraph::new(Text::from(transform_digit_to_ascii(hour_2)));
    let paragraph_3 = Paragraph::new(Text::from(draw_colon()));
    let paragraph_4 = Paragraph::new(Text::from(transform_digit_to_ascii(minute_1)));
    let paragraph_5 = Paragraph::new(Text::from(transform_digit_to_ascii(minute_2)));


    f.render_widget(paragraph_1, left_inner_upper_chunks[1]);
    f.render_widget(paragraph_2, left_inner_upper_chunks[2]);
    f.render_widget(paragraph_3, left_inner_upper_chunks[3]);
    f.render_widget(paragraph_4, left_inner_upper_chunks[4]);
    f.render_widget(paragraph_5, left_inner_upper_chunks[5]);

    

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),

            CurrentScreen::EditingStarttime => {
                Span::styled("Normal Mode", Style::default().fg(Color::DarkGray))
            }

            CurrentScreen::EditingEndtime => {
                Span::styled("Normal Mode", Style::default().fg(Color::DarkGray))
            }

            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Starttime => {
                        Span::styled("Editing Starttime", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Endtime => {
                        Span::styled("Editing Endtime", Style::default().fg(Color::Green))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (s) to edit Starttime / (e) to edit Endtime",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::EditingStarttime => Span::styled(
                "(ESC) to cancel/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::EditingEndtime => Span::styled(
                "(ESC) to cancel/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "(q) to quit / (s) to edit Starttime / (e) to edit Endtime",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(editing) = &app.currently_editing {

        let percent_x = 60;
        let percent_y = 30;

        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(left_inner_chunks[1]);
    
        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1];

        //let area = centered_rect(30, 15, f.size());

        let mut key_block = Block::default().title("Starttime").borders(Borders::ALL);

        let mut value_block = Block::default().title("Endtime").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Starttime => {
                key_block = key_block.style(active_style);
                let key_text = Paragraph::new(app.starttime_input.clone()).block(key_block);
                f.render_widget(key_text, area);
            }

            CurrentlyEditing::Endtime => {
                value_block = value_block.style(active_style);
                let value_text = Paragraph::new(app.endtime_input.clone()).block(value_block);
                f.render_widget(value_text, area);
            }
        };
    }

    if let CurrentScreen::Exiting = app.current_screen {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Exit Confirmation")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        let exit_text = Text::styled(
            "Would you like to output the buffer and save the worktime as json? (y/n)",
            Style::default().fg(Color::Red).add_modifier(Modifier::ITALIC),
        );

        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());

        f.render_widget(exit_paragraph, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
