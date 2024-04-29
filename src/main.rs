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
use std::io::{stdin, stdout, Write};
use time_tracking::app::{App, CurrentScreen, CurrentlyEditing};
use time_tracking::ui::ui;

//fn main () -> io::Result<()> {
//    let (Key, Values) = get_starttime();
//    let (Keys_tmp, Values_tmp) = calc_endtime(Key, Values);
//    let now = Local::now();
//    let now = now.format("%H:%M").to_string();
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
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

//fn parse_time(time: String) -> (i32, i32) {
//    //std::io::stdin().read_line(&mut time).expect("read_line error");
//    let mut time_str = time.split(":");
//    let hour: i32 = time_str.next().unwrap().trim().parse().unwrap();
//    let minutes: i32 = time_str.next().unwrap().trim().parse().unwrap();
//    (hour, minutes)
//}

//fn calc_endtime(hour: i32, minutes: i32) -> (i32, i32) {
//    let mut hour_tmp: i32 = hour + 7;
//    let mut minutes_tmp: i32 = minutes + 80;
//
//    while minutes_tmp > 60 {
//        hour_tmp += 1;
//        minutes_tmp -= 60;
//    }
//
//    (hour_tmp, minutes_tmp)
//}

//fn get_starttime() -> (i32, i32) {
//    let mut time = String::new();
//    println!("Please enter your start time: ");
//    let _ = stdout().flush();
//    stdin()
//        .read_line(&mut time)
//        .expect("Please use the format Key:Values");
//    println!("You entered: {time}");
//    let (hour, minutes) = parse_time(time);
//    (hour, minutes)
//}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue; // Skip events that are not KeyEventKind::Press
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;

                        app.currently_editing = Some(CurrentlyEditing::Key);
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

                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.currently_editing = Some(CurrentlyEditing::Value);
                                }

                                CurrentlyEditing::Value => {
                                    app.save_key_value();

                                    app.current_screen = CurrentScreen::Main;
                                }
                            }
                        }
                    }

                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.pop();
                                }
                                CurrentlyEditing::Value => {
                                    app.value_input.pop();
                                }
                            }
                        }
                    }

                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;

                        app.currently_editing = None;
                    }

                    KeyCode::Tab => {
                        app.toggle_editing();
                    }

                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Key => {
                                    app.key_input.push(value);
                                }

                                CurrentlyEditing::Value => {
                                    app.value_input.push(value);
                                }
                            }
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
