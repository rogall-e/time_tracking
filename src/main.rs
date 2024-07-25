use crossterm::event::{self, KeyCode, KeyEventKind};
use std::io::Result;
use time_tracking_basic::app::{App, CurrentScreen, CurrentlyEditing, Tab};
use time_tracking_basic::tui::{Event, Tui};
use time_tracking_basic::ui::ui;
use time_tracking_basic::calc_time::parse_time;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // create folder to save the data
    if !std::path::Path::new("data").exists() {
        std::fs::create_dir_all("data")?;
    }
    if !std::path::Path::new(".tmp_cache").exists() {
        std::fs::create_dir_all(".tmp_cache")?;
        std::fs::File::create(".tmp_cache/focus_cache.bin")?;
        let mut focus_cache_file = std::fs::File::create(".tmp_cache/focus_cache.bin").unwrap();
        let export_focus: String = "false".to_string() + &','.to_string() + "0";
        focus_cache_file.write_all(export_focus.as_bytes()).unwrap();
        let mut list_cache_file = std::fs::File::create(".tmp_cache/list_cache.bin").unwrap();
        let export: String = "0".to_string();
        list_cache_file.write_all(export.as_bytes()).unwrap();
    } else {
        let mut focus_cache_file = std::fs::File::create(".tmp_cache/focus_cache.bin").unwrap();
        let export_focus: String = "false".to_string() + &','.to_string() + "0";
        focus_cache_file.write_all(export_focus.as_bytes()).unwrap();
        let mut list_cache_file = std::fs::File::create(".tmp_cache/list_cache.bin").unwrap();
        let export: String = "0".to_string();
        list_cache_file.write_all(export.as_bytes()).unwrap();
    }

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
    let mut counter_focus = 0;
    loop {
        let event = tui.next().await?;
        if let Event::Render = event.clone() {
            tui.draw(|f| ui(f, &mut app))?;
        };

        if let Event::Tick = event.clone() {
            if app.meeting_running {
                counter += 1;
                if counter == 60 {
                    app.time_in_meetings += 1;
                    counter = 0;
                }
            }
            if app.focus {
                counter_focus += 1;
                if counter_focus == 60 {
                    app.focus_time += 1;
                    app.chache_focus_time();
                    counter_focus = 0;
                }
            }
            let current_time = chrono::Local::now().format("%H:%M").to_string();
            if app.starttime_pairs.is_empty() {
                let start_time = parse_time(&app.default_starttime);
                let start_minutes = start_time.0 * 60 + start_time.1;
                let current_time = parse_time(&current_time);
                let current_minutes = current_time.0 * 60 + current_time.1;
                let current_worktime = current_minutes - start_minutes;
                app.current_worktime = current_worktime as u64
            } 
            if app.starttime_pairs.contains_key(&app.starttime_key) {
                let start_time = parse_time(&app.starttime_pairs[&app.starttime_key]);
                let start_minutes = start_time.0 * 60 + start_time.1;
                let current_time = parse_time(&current_time);
                let current_minutes = current_time.0 * 60 + current_time.1;
                let current_worktime = current_minutes - start_minutes;
                app.current_worktime = current_worktime as u64
            }

            app.total_time_in_meetings = app.meeting_list
                .clone()
                .into_iter()
                .map(|x| x.time_in_meeting)
                .sum();

            if app.meeting_running {
                app.total_time_in_meetings += app.time_in_meetings;
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

                    KeyCode::Char('r') => {
                        app.next_tab();
                    }
                    
                    KeyCode::Char('l') => {
                        app.previous_tab();
                    } 
                    
                    KeyCode::Right => {
                        app.horizontal_scroll = app.horizontal_scroll.saturating_add(1);
                        app.scrollbar_state = app.scrollbar_state.position(app.horizontal_scroll);
                    }
                    
                    KeyCode::Left => {
                        app.horizontal_scroll = app.horizontal_scroll.saturating_sub(1);
                        app.scrollbar_state = app.scrollbar_state.position(app.horizontal_scroll);
                    }
                    
                   KeyCode::Down => {
                        match app.tab {
                            Tab::BarChartTab => {
                                continue;
                            }
                            Tab::EditHistoryTab => {
                               app.next_list_item();
                            }
                            Tab::FocusTimeTab => {
                                continue;
                            }
                            Tab::MeetingNotesTab => {
                                continue;
                            }
                        }
                    }

                    KeyCode::Up => {
                        match app.tab {
                            Tab::BarChartTab => {
                                continue;
                            }
                            Tab::EditHistoryTab => {
                               app.previous_list_item();
                            }
                            Tab::FocusTimeTab => {
                                continue;
                            }
                            Tab::MeetingNotesTab => {
                                continue;
                            }
                        }
                    }

                    KeyCode::Char('F') => {
                        match app.tab {
                            Tab::BarChartTab => {
                                continue;
                            }
                            Tab::EditHistoryTab => {
                                continue;
                            }
                            Tab::FocusTimeTab => {
                                if !app.focus {
                                    app.start_focus_time();
                                    app.chache_focus_time();
                                    
                                } else {
                                    app.end_focus_time();
                                    app.chache_focus_time();
                                }
                            }
                            Tab::MeetingNotesTab => {
                                continue;
                            }
                        }
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
                        app.current_screen = CurrentScreen::Main
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
                                app.start_meeting();
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
