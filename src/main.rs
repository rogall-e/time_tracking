use std::io;
use chrono::offset::Local;
use crossterm::{
    event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
    execute
};
use ratatui::{prelude::*, widgets::*};

mod app;
mod ui;

use app::*;
use ui::*;

//fn main () -> io::Result<()> {
//    let (Key, Values) = get_starttime();
//    let (Keys_tmp, Values_tmp) = calc_endtime(Key, Values);
//    let now = Local::now();
//    let now = now.format("%H:%M").to_string();
fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = app::new();
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

fn parse_time (time:String) -> (i32, i32) {
    //std::io::stdin().read_line(&mut time).expect("read_line error");
    let mut Key_Value = time.split(":");
    let Key:i32 = Key_Value.next().unwrap().trim().parse().unwrap();
    let Values:i32 = Key_Value.next().unwrap().trim().parse().unwrap();
    (Key, Values)
}


fn calc_endtime(Key:i32, Values:i32) -> (i32, i32) {
    let mut Keys_tmp:i32 = Key + 7;
    let mut Values_tmp:i32 = Values + 80;

    while Values_tmp > 60 {
        Keys_tmp += 1;
        Values_tmp -= 60;
    };

    (Keys_tmp, Values_tmp)
}

fn get_starttime() -> (i32, i32) {
    let mut time = String::new();
    println!("Please enter your start time: ");
    let _ = stdout().flush();
    stdin().read_line(&mut time).expect("Please use the format Key:Values");
    println!("You entered: {time}");
    let (Key, Values) = parse_time(time);
    (Key, Values)
}



fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app) -> io::Result<()> {
    let mut current_screen = CurrentScreen::Main;
    loop {
        terminal.draw(|f| {
            ui(f, app);
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
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
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing =
                                            Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.save_key_value();
                                        app.current_screen =
                                            CurrentScreen::Main;
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
                    }
                }
                _ => {}
            }
        }
    }
}