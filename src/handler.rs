use crate::app::{App, AppResult, BookEditFocus, BookState, EDIT_WINDOW_FOCUS};
use crate::database;
use crate::database::models::NewBook;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use int_enum::IntEnum;
use std::error;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders};
use tui_textarea::TextArea;

pub fn change_focus(task: &mut BookState<'_>, forward: bool) -> Result<(), Box<dyn error::Error>> {
    let cycle = if forward {
        (task.focus.int_value() + 1) % EDIT_WINDOW_FOCUS
    } else {
        let mut current_value = (task.focus.int_value() - 1) % EDIT_WINDOW_FOCUS;
        if current_value < 0 {
            current_value = 6;
        }
        current_value
    };
    task.focus = BookEditFocus::from_int(cycle)?;
    Ok(())
}

// Put this function in ui.rs
fn validate(textarea: &mut TextArea) -> bool {
    if let Err(err) = textarea.lines()[0].parse::<f64>() {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("ERROR: {}", err)),
        );
        false
    } else {
        textarea.set_style(Style::default().fg(Color::LightGreen));
        textarea.set_block(Block::default().borders(Borders::ALL).title("OK"));
        true
    }
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.book_edit_state.is_some() {
        let updated_task = if let Some(mut task) = app.book_edit_state.take() {
            match (key_event.code, task.focus) {
                (KeyCode::Tab, _) => {
                    change_focus(&mut task, true)?;
                    Some(task)
                }
                (KeyCode::BackTab, _) => {
                    change_focus(&mut task, false)?;
                    Some(task)
                }

                (KeyCode::Enter, BookEditFocus::ConfirmBtn) => {
                    let title = task.title.into_lines().join("\n");
                    let author = task.author.into_lines().join("\n");
                    let genre = task.genre.into_lines().join("\n");
                    let status = task.status.into_lines().join("\n");
                    //rewrite this mess
                    let validate_rating = validate(&mut task.rating);
                    let mut rating = 0;

                    if validate_rating {
                        rating = task.rating.lines()[0].parse::<i32>().unwrap();
                        if rating > 10 {
                            rating = 10;
                        }
                    } else {
                        rating = 0;
                    }

                    let new_book = NewBook {
                        title,
                        author,
                        genre,
                        rating,
                        status,
                    };

                    database::create_book(new_book);
                    app.show_popup = !app.show_popup;
                    None
                }
                (KeyCode::Enter, BookEditFocus::CancelBtn) => {
                    app.show_popup = !app.show_popup;
                    None
                }
                (KeyCode::Enter, BookEditFocus::Title) => Some(task),
                (_, BookEditFocus::Title) => {
                    task.title.input(key_event);
                    Some(task)
                }
                (_, BookEditFocus::Author) => {
                    task.author.input(key_event);
                    Some(task)
                }
                (_, BookEditFocus::Genre) => {
                    task.genre.input(key_event);
                    Some(task)
                }
                (_, BookEditFocus::Rating) => {
                    task.rating.input(key_event);
                    Some(task)
                }
                (_, BookEditFocus::Status) => {
                    task.status.input(key_event);
                    Some(task)
                }
                _ => Some(task),
            }
        } else {
            None
        };
        app.book_edit_state = updated_task;
    } else {
        match key_event.code {
            // Exit application on `ESC` or `q`
            KeyCode::Esc | KeyCode::Char('q') => {
                app.quit();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            // Remove book
            KeyCode::Char('d') => {
                if app.items.len() != 0 {
                    database::delete_book(app);
                }
            }
            KeyCode::Char('a') => {
                app.book_edit_state = Some(BookState::default());
                app.show_popup = !app.show_popup;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.items.len() != 0 {
                    app.previous();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.items.len() != 0 {
                    app.next();
                }
            }
            KeyCode::Char('?') => {
                app.help_popup = !app.help_popup;
            }
            _ => {}
        }
    }
    Ok(())
}
