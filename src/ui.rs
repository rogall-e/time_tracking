
use crate::app::{App, CurrentScreen, CurrentlyEditing};
 use chrono::Local;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    symbols::scrollbar,
    widgets::{Block, 
        Borders, 
        Clear, 
        List, 
        ListItem, 
        Paragraph, 
        Wrap, 
        Scrollbar, 
        ScrollbarOrientation
    },
    Frame,
};
use tui_big_text::{BigTextBuilder, PixelSize};

use crate::barchart::{BarChartApp, draw_bar_with_group_labels};

pub fn ui(f: &mut Frame<'_>, app: &mut App) {
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
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(inner_chunks[0]);

    let left_inner_lower_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(left_inner_chunks[1]);

    let start_endtime_chunck = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(left_inner_lower_chunks[0]);

    let barchart_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(84), Constraint::Percentage(16)])
        .split(inner_chunks[1]);

    // Title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let title = Paragraph::new(Text::styled(
        "Time Tracker",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    // List of Starttime and Endtime

    let mut starttime_list_items = Vec::<ListItem>::new();
    let mut endtime_list_items = Vec::<ListItem>::new();

    for key in app.starttime_pairs.keys() {
        starttime_list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.starttime_pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    for key in app.endtime_pairs.keys() {
        endtime_list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.endtime_pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let starttime_list = List::new(starttime_list_items).block(
        Block::default()
            .title("Worktime")
            .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
            .style(Style::default().fg(Color::White)),
    );

    let endtime_list = List::new(endtime_list_items).block(
        Block::default()
            .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
            .style(Style::default().fg(Color::White)),
    );

    f.render_widget(starttime_list, start_endtime_chunck[0]);
    f.render_widget(endtime_list, start_endtime_chunck[1]);

    // List of Meetings
    let mut meeting_list_items = Vec::<ListItem>::new();

    for meeting in &app.meeting_list {
        meeting_list_items.push(ListItem::new(Line::from(Span::styled(
            format!(
                "{: <25} : {} - {} ({} min)",
                meeting.meeting_name,
                meeting.meeting_start_time,
                meeting.meeting_end_time,
                meeting.time_in_meeting
            ),
            Style::default().fg(Color::Yellow),
        ))));
    }

    if app.meeting_running {
        meeting_list_items.push(ListItem::new(Line::from(Span::styled(
            format!(
                "{: <25} : {} - Ongoing Meeting ({} min)",
                app.meeting_name, app.meeting_start_time, app.time_in_meetings
            ),
            Style::default().fg(Color::Yellow),
        ))));
    };

    let meeting_list = List::new(meeting_list_items).block(
        Block::default()
            .title("Meetings")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White)),
    );

    f.render_widget(meeting_list, left_inner_lower_chunks[1]);

    // Worktime Barchart
    // Load Json Data
    let barchart_app = BarChartApp::new();
    
    let barchart = draw_bar_with_group_labels(&barchart_app, false);
   
    f.render_widget(barchart, barchart_chunk[0]);

    // bar for current day
    let current_date = Local::now().format("%Y-%m-%d").to_string();

    let barchart_app_today = BarChartApp::new_current(app.current_worktime, app.total_time_in_meetings as u64, current_date);
    let barchart_today = draw_bar_with_group_labels(&barchart_app_today, true);

    f.render_widget(barchart_today, barchart_chunk[1]);
   
    // Clock
    let current_time = Local::now().format("%H:%M").to_string();
    if f.size().width > 100 && f.size().height > 30 {
        let big_text = BigTextBuilder::default()
            .pixel_size(PixelSize::Full)
            .style(Style::new().fg(Color::Green))
            .lines(vec![current_time.into()])
            .build()
            .unwrap();
        f.render_widget(big_text, left_inner_chunks[0]);
    } else {
        let small_text = BigTextBuilder::default()
            .pixel_size(PixelSize::Quadrant)
            .style(Style::new().fg(Color::Green))
            .lines(vec![current_time.into()])
            .build()
            .unwrap();

        f.render_widget(small_text, left_inner_chunks[0]);
    }

    // Navigation text
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

            CurrentScreen::EditingMeetingName => {
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
                    CurrentlyEditing::MeetingName => {
                        Span::styled("Editing Meeting Name", Style::default().fg(Color::Green))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::White)));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "Press (q) to quit | (s) to edit Starttime | (e) to edit Endtime | (m) start Meeting | (M) stop Meeting",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::EditingStarttime => Span::styled(
                "(ESC) to cancel | (enter) to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::EditingEndtime => Span::styled(
                "(ESC) to cancel | (enter) to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::EditingMeetingName => Span::styled(
                "(ESC) to cancel | (enter) to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exiting => Span::styled(
                "Press (q) to quit | (s) to edit Starttime | (e) to edit Endtime | (m) start/stop Meeting | (M) stop Meeting",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::White)));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    // Editing mode
    if let Some(editing) = &app.currently_editing {
        // Layout for the editing mode
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

        let mut key_block = Block::default().title("Starttime").borders(Borders::ALL).style(Style::default().fg(Color::White));

        let mut value_block = Block::default().title("Endtime").borders(Borders::ALL).style(Style::default().fg(Color::White));

        let mut meeting_block = Block::default().title("Meeting Name").borders(Borders::ALL).style(Style::default().fg(Color::White));

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

            CurrentlyEditing::MeetingName => {
                meeting_block = meeting_block.style(active_style);
                let value_text =
                    Paragraph::new(app.meeting_name_input.clone()).block(meeting_block);
                f.render_widget(value_text, area);
            }
        };
    }

    // Exit confirmation
    if let CurrentScreen::Exiting = app.current_screen {
        f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Exit Confirmation")
            .borders(Borders::NONE)
            .style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        let exit_text = Text::styled(
            "Would you like to output the buffer and save the worktime as json? (y/n)",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::ITALIC),
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
