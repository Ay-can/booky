use crate::app::{App, AppResult, BookEditFocus, BookState, InputMode, EDIT_WINDOW_FOCUS};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use int_enum::IntEnum;
use std::error;

pub fn change_focus(task: &mut BookState<'_>, forward: bool) -> Result<(), Box<dyn error::Error>> {
    let cycle = if forward {
        (task.focus.int_value() + 1) % EDIT_WINDOW_FOCUS
    } else {
        (task.focus.int_value() - 1) % EDIT_WINDOW_FOCUS
    };
    task.focus = BookEditFocus::from_int(cycle)?;
    Ok(())
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
                    app.write_json(title, author);
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
                    app.remove_json_at_index().expect("Failed to remove");
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
            _ => {}
        }
    }
    Ok(())
}
