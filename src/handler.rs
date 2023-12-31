use crate::app::{
    App, AppResult, BookEditFocus, BookState, SearchFieldFocus, SearchState, EDIT_WINDOW_FOCUS,
    SEARCH_WINDOW_FOCUS,
};
use crate::database;
use crate::database::models::NewBook;
use chrono::{Local, NaiveDate};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use int_enum::IntEnum;
use std::error;

use tui_textarea::TextArea;

// This function allows us to change the focus when pressing tab in the add/update menu
pub fn change_add_focus(
    task: &mut BookState<'_>,
    forward: bool,
) -> Result<(), Box<dyn error::Error>> {
    let cycle = if forward {
        (task.focus.int_value() + 1) % EDIT_WINDOW_FOCUS
    } else {
        let mut current_value = (task.focus.int_value() - 1) % EDIT_WINDOW_FOCUS;
        if current_value < 0 {
            current_value = 8;
        }
        current_value
    };
    task.focus = BookEditFocus::from_int(cycle)?;
    Ok(())
}

// This function does the same as above but for the search menu
pub fn change_search_focus(
    task: &mut SearchState<'_>,
    forward: bool,
) -> Result<(), Box<dyn error::Error>> {
    let cycle = if forward {
        (task.focus.int_value() + 1) % SEARCH_WINDOW_FOCUS
    } else {
        let mut current_value = (task.focus.int_value() - 1) % SEARCH_WINDOW_FOCUS;
        if current_value < 0 {
            current_value = 8;
        }
        current_value
    };
    task.focus = SearchFieldFocus::from_int(cycle)?;
    Ok(())
}

pub fn handle_add_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let updated_task = if let Some(mut task) = app.book_edit_state.take() {
        match (key_event.code, task.focus) {
            (KeyCode::Tab, _) => {
                change_add_focus(&mut task, true)?;
                Some(task)
            }
            (KeyCode::BackTab, _) => {
                change_add_focus(&mut task, false)?;
                Some(task)
            }
            (KeyCode::Enter, BookEditFocus::ConfirmBtn) => {
                let title = task.title.into_lines().join("\n");
                let author = task.author.into_lines().join("\n");
                let genre = task.genre.into_lines().join("\n");
                let status = task.status.into_lines().join("\n");
                let start_date = task.start_date.into_lines().join("\n");
                let end_date = task.end_date.into_lines().join("\n");
                let rating = task.rating.lines()[0].parse::<i32>().unwrap_or_default();

                let default_date = Local::now().naive_local();
                let start_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
                    .unwrap_or(default_date.into());
                let end_date =
                    NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap_or(default_date.into());

                let new_book = NewBook {
                    title,
                    author,
                    genre,
                    rating,
                    status,
                    start_date: Some(start_date),
                    end_date: Some(end_date),
                };
                if !task.is_edit {
                    database::create_book(new_book);
                } else {
                    if let Some(selected) = app.state.selected() {
                        let current_id = app.items.get(selected).unwrap().id;
                        database::update_book(current_id, new_book);
                    }
                }
                app.add_popup = !app.add_popup;
                None
            }
            (KeyCode::Enter, BookEditFocus::CancelBtn) => {
                app.add_popup = !app.add_popup;
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
            (_, BookEditFocus::StartDate) => {
                task.start_date.input(key_event);
                Some(task)
            }
            (_, BookEditFocus::EndDate) => {
                task.end_date.input(key_event);
                Some(task)
            }
            _ => Some(task),
        }
    } else {
        None
    };
    app.book_edit_state = updated_task;
    Ok(())
}

pub fn handle_search_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let updated_task = if let Some(mut task) = app.search_field_state.take() {
        match (key_event.code, task.focus) {
            (KeyCode::Tab, _) => {
                change_search_focus(&mut task, true)?;
                Some(task)
            }
            (KeyCode::BackTab, _) => {
                change_search_focus(&mut task, false)?;
                Some(task)
            }
            (KeyCode::Enter, SearchFieldFocus::Title) => Some(task),
            (_, SearchFieldFocus::Title) => {
                task.title.input(key_event);
                Some(task)
            }
            (_, SearchFieldFocus::Author) => {
                task.author.input(key_event);
                Some(task)
            }
            (_, SearchFieldFocus::Genre) => {
                task.genre.input(key_event);
                Some(task)
            }
            (_, SearchFieldFocus::Rating) => {
                task.rating.input(key_event);
                Some(task)
            }
            (_, SearchFieldFocus::Status) => {
                task.status.input(key_event);
                Some(task)
            }
            (_, SearchFieldFocus::StartDate) => {
                task.start_date.input(key_event);
                Some(task)
            }
            (_, SearchFieldFocus::EndDate) => {
                task.end_date.input(key_event);
                Some(task)
            }
            (KeyCode::Enter, SearchFieldFocus::ConfirmBtn) => {
                let title = task.title.into_lines().join("\n");
                let author = task.author.into_lines().join("\n");
                let genre = task.genre.into_lines().join("\n");
                let rating = task.rating.lines()[0].parse::<i32>().unwrap_or_default();
                let status = task.status.into_lines().join("\n");
                let start_date = task.start_date.into_lines().join("\n");
                let end_date = task.end_date.into_lines().join("\n");

                // If the user doesn't provide a date to search/filter
                // default to searching for a book that has a startdate of > 1500
                // end an end date of 4050 <
                // Startdate doesn't mean releasedate, instead it means the date you read
                // the book, the reason I do this is because other wise it doesn't filter
                // correctly in database.rs
                // If you are reading this from 4050 I'm sorry.

                let default_start_date = NaiveDate::from_ymd_opt(1500, 1, 1).unwrap();
                let default_end_date = NaiveDate::from_ymd_opt(4050, 1, 1).unwrap();
                let start_date = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
                    .unwrap_or(default_start_date.into());
                let end_date = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d")
                    .unwrap_or(default_end_date.into());

                let book_info = NewBook {
                    title,
                    author,
                    genre,
                    rating,
                    status,
                    start_date: Some(start_date),
                    end_date: Some(end_date),
                };

                app.items = database::search_book(book_info);
                app.search_active = true;
                app.search_popup = !app.search_popup;
                None
            }
            (KeyCode::Enter, SearchFieldFocus::CancelBtn) => {
                app.search_popup = !app.search_popup;
                None
            }
            _ => Some(task),
        }
    } else {
        None
    };
    app.search_field_state = updated_task;
    Ok(())
}

pub fn handle_main_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
        KeyCode::Char('u') => {
            // Do this in app later?
            if let Some(index) = app.state.selected() {
                let current_book = app.items.get(index).unwrap();
                let new_book_state = BookState {
                    title: TextArea::from(current_book.title.lines()),
                    author: TextArea::from(current_book.author.lines()),
                    genre: TextArea::from(current_book.genre.lines()),
                    rating: TextArea::from(current_book.rating.to_string().lines()),
                    status: TextArea::from(current_book.status.lines()),
                    start_date: TextArea::from(
                        current_book.start_date.unwrap().to_string().lines(),
                    ),
                    end_date: TextArea::from(current_book.end_date.unwrap().to_string().lines()),
                    focus: BookEditFocus::Title,
                    is_edit: true,
                };
                app.book_edit_state = Some(new_book_state);
            }
            app.add_popup = !app.add_popup;
        }
        KeyCode::Char('i') => {
            app.book_edit_state = Some(BookState::default());
            app.search_active = false;
            app.add_popup = !app.add_popup;
        }
        // Clear search query
        KeyCode::Char('r') => {
            app.search_active = false;
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
        KeyCode::Char('/') => {
            app.search_field_state = Some(SearchState::default());
            app.search_popup = !app.search_popup;
        }
        KeyCode::Char('?') => {
            app.help_popup = !app.help_popup;
        }
        _ => {}
    }
    Ok(())
}

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    // Only handle Press or Repeat events, ignore Release events
    // Without this check terminal will register two events for each key press on some platforms
    if key_event.kind == KeyEventKind::Press || key_event.kind == KeyEventKind::Repeat {
        if app.book_edit_state.is_some() {
            handle_add_events(key_event, app).expect("Failed to handle events related to adding");
        } else if app.search_field_state.is_some() {
            handle_search_events(key_event, app)
                .expect("Failed to handle events related to searching");
        } else {
            handle_main_events(key_event, app).expect("Failed to handle main events");
        }
    }
    Ok(())
}
