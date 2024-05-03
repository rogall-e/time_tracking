use crate::app::{App, CurrentScreen, CurrentlyEditing};
use crate::calc_time::parse_time;
use crate::read_json::read_json;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{BarChart, Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

struct JsonParsed<'a> {
    data: Vec<(&'a str, i32)>,
}

impl<'a> JsonParsed<'a> {
    fn new() -> Self {
        let worktime_list = read_json();
        let mut worktime_items = vec![];
        for worktime in worktime_list {
            let date: &str = worktime.date.as_str();
            let start_time = parse_time(&worktime.starttime);
            let end_time = parse_time(&worktime.endtime);
            let start_minutes = start_time.0 * 60 + start_time.1;
            let end_minutes = end_time.0 * 60 + end_time.1;
            let worktime_in_min: i32 = end_minutes - start_minutes;
            worktime_items.push((&date, worktime_in_min));
        }

        JsonParsed {
            data: worktime_items,
        }
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

    let list = List::new(list_items);

    f.render_widget(list, inner_chunks[0]);

    let barchart = BarChart::default()
        .block(Block::default().title("Worktime").borders(Borders::ALL))
        .data(&JsonParsed::new().data)
        .bar_width(5)
        .bar_gap(1)
        .value_style(Style::default().fg(Color::White).bg(Color::Green))
        .label_style(Style::default().fg(Color::Yellow))
        .style(Style::default().fg(Color::White));

    f.render_widget(barchart, inner_chunks[1]);

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
        let area = centered_rect(60, 25, f.size());

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
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer and save the worktime as json? (y/n)",
            Style::default().fg(Color::Red),
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
