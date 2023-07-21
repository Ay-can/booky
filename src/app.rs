use crate::database::models::Book;
use int_enum::IntEnum;
use std::error;

use tui::widgets::TableState;
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub const EDIT_WINDOW_FOCUS: i8 = 8;
pub const SEARCH_WINDOW_FOCUS: i8 = 3;

#[repr(i8)]
#[derive(Debug, IntEnum, Clone, Copy)]
pub enum SearchFieldFocus {
    Input = 0,
    ConfirmBtn = 1,
    CancelBtn = 2,
}

pub struct SearchState<'a> {
    pub input: TextArea<'a>,
    pub focus: SearchFieldFocus,
}

impl Default for SearchState<'_> {
    fn default() -> Self {
        SearchState {
            input: TextArea::default(),
            focus: SearchFieldFocus::Input,
        }
    }
}

#[repr(i8)]
#[derive(Debug, IntEnum, Clone, Copy)]
pub enum BookEditFocus {
    Title = 0,
    Author = 1,
    Genre = 2,
    Rating = 3,
    Status = 4,
    StartDate = 5,
    ConfirmBtn = 6,
    CancelBtn = 7,
}

pub struct BookState<'a> {
    pub title: TextArea<'a>,
    pub author: TextArea<'a>,
    pub genre: TextArea<'a>,
    pub rating: TextArea<'a>,
    pub status: TextArea<'a>,
    pub start_date: TextArea<'a>,
    pub focus: BookEditFocus,
    pub is_edit: bool,
}

impl Default for BookState<'_> {
    fn default() -> Self {
        BookState {
            title: TextArea::default(),
            author: TextArea::default(),
            genre: TextArea::default(),
            rating: TextArea::default(),
            status: TextArea::default(),
            start_date: TextArea::default(),
            focus: BookEditFocus::Title,
            is_edit: false,
        }
    }
}

/// Application.
pub struct App<'a> {
    pub running: bool,
    pub add_popup: bool,
    pub help_popup: bool,
    pub search_popup: bool,
    pub search_active: bool,
    pub state: TableState,
    pub book_edit_state: Option<BookState<'a>>,
    pub search_field_state: Option<SearchState<'a>>,
    pub items: Vec<Book>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            running: true,
            add_popup: false,
            help_popup: false,
            search_popup: false,
            search_active: false,
            state: TableState::default(),
            book_edit_state: None,
            search_field_state: None,
            items: Vec::new(),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
