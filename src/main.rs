use crossterm::event::{self, KeyCode, KeyEventKind};
use std::io::Result;
use time_tracking_basic::app::{App, CurrentScreen, CurrentlyEditing};
use time_tracking_basic::tui::{Event, Tui};
use time_tracking_basic::ui::ui;

#[tokio::main]
async fn main() -> Result<()> {
    // create app and run it
    let res = run_app().await;
    res?;
    Ok(())
}

async fn run_app() -> Result<()> {
    let mut tui = Tui::new()?;
    tui.enter()?;

    let mut app = App::new();
    let mut counter = 0;
    loop {
        let event = tui.next().await?;
        if let Event::Render = event.clone() {
            tui.draw(|f| ui(f, &app))?;
        };

        if let Event::Tick = event.clone() {
            if app.meeting_running {
                counter += 1;
                if counter == 60 {
                    app.time_in_meetings += 1;
                    counter = 0;
                }
            }
        }

        if let Event::Key(key) = event.clone() {
            if key.kind == event::KeyEventKind::Release {
                continue; // Skip events that are not KeyEventKind::Press
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('s') => {
                        app.current_screen = CurrentScreen::EditingStarttime;
                        app.currently_editing = Some(CurrentlyEditing::Starttime);
                    }

                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::EditingEndtime;
                        app.currently_editing = Some(CurrentlyEditing::Endtime);
                    }

                    KeyCode::Char('m') => {
                        app.current_screen = CurrentScreen::EditingMeetingName;
                        app.currently_editing = Some(CurrentlyEditing::MeetingName);
                    }

                    KeyCode::Char('M') => {
                        app.end_meeting();
                    }

                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }

                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        app.do_print = true;
                        app.should_exit = true;
                        let export_result = app.export_json();
                        if let Err(e) = export_result {
                            eprintln!("Error exporting JSON: {}", e);
                        }
                    }

                    KeyCode::Char('n') => {
                        app.should_exit = true;
                    }

                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },

                CurrentScreen::EditingStarttime if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(CurrentlyEditing::Starttime) = &app.currently_editing {
                                app.save_starttime_value();
                                app.current_screen = CurrentScreen::Main;
                            }
                        }

                        KeyCode::Backspace => {
                            if let Some(CurrentlyEditing::Starttime) = &app.currently_editing {
                                app.starttime_input.pop();
                            }
                        }

                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }

                        KeyCode::Char(value) => {
                            if let Some(CurrentlyEditing::Starttime) = &app.currently_editing {
                                app.starttime_input.push(value);
                            }
                        }
                        _ => {}
                    }
                }
                CurrentScreen::EditingEndtime if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(CurrentlyEditing::Endtime) = &app.currently_editing {
                                app.save_endtime_value();
                                app.current_screen = CurrentScreen::Main;
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(CurrentlyEditing::Endtime) = &app.currently_editing {
                                app.endtime_input.pop();
                            }
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        KeyCode::Char(value) => {
                            if let Some(CurrentlyEditing::Endtime) = &app.currently_editing {
                                app.endtime_input.push(value);
                            }
                        }
                        _ => {}
                    }
                }
                CurrentScreen::EditingMeetingName if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(CurrentlyEditing::MeetingName) = &app.currently_editing {
                                app.save_meeting_name();
                                app.current_screen = CurrentScreen::Main;
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(CurrentlyEditing::MeetingName) = &app.currently_editing {
                                app.meeting_name_input.pop();
                            }
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        KeyCode::Char(value) => {
                            if let Some(CurrentlyEditing::MeetingName) = &app.currently_editing {
                                app.meeting_name_input.push(value);
                            }
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
        if app.should_exit {
            break;
        }
    }
    tui.exit()?;
    Ok(())
}
