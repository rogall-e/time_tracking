//use chrono::offset::Local;
use anyhow::Result;
use crossterm::event::DisableMouseCapture;
use crossterm::event::{self, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::io;
use time_tracking_basic::app::{App, CurrentScreen, CurrentlyEditing};
use time_tracking_basic::ui::ui;

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;

    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine

    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);

    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
            app.export_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
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
                        app.currently_editing = Some(CurrentlyEditing::Endtime)
                    }

                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }

                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }

                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
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

                _ => {}
            }
        }
    }
}
