use serde::{Deserialize, Serialize};
use std::error;
use std::fs;
use tui::widgets::TableState;
use tui_textarea::TextArea;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const JSON_PATH: &str = "../data/books.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub id: usize,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub rating: usize,
}

pub enum BookEditFocus {
    Title,
    Author,
    ConfirmBtn,
    CancelBtn,
}

pub struct BookState<'a> {
    pub title: TextArea<'a>,
    pub author: TextArea<'a>,
    pub focus: BookEditFocus,
    pub is_edit: bool,
}

impl Default for BookState<'_> {
    fn default() -> Self {
        BookState {
            title: TextArea::default(),
            author: TextArea::default(),
            focus: BookEditFocus::Title,
            is_edit: false,
        }
    }
}

/// Application.
pub struct App<'a> {
    pub running: bool,
    pub show_popup: bool,
    pub state: TableState,
    pub book_edit_state: Option<BookState<'a>>,
    pub items: Vec<Book>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            running: true,
            show_popup: false,
            state: TableState::default(),
            book_edit_state: None,
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

    // Rewrite
    pub fn read_json(&mut self) -> Result<Vec<Book>, Box<dyn error::Error>> {
        let json_content = fs::read_to_string(JSON_PATH)?;
        let parsed: Vec<Book> = serde_json::from_str(&json_content)?;
        // This is a yucky fix :(
        // Change by moving to a module so I can use it in default()?
        self.items = parsed.clone();
        Ok(parsed)
    }
    pub fn remove_json_at_index(&mut self) -> Result<(), Box<dyn error::Error>> {
        // If the selected state glitches when removed it is because
        // of these lines. Instead of deleting the state create a parsed vec
        // like we did above
        if let Some(selected) = self.state.selected() {
            self.items.remove(selected);
            fs::write(JSON_PATH, &serde_json::to_vec(&self.items)?)?;

            if selected > 0 {
                self.state.select(Some(selected - 1))
            } else {
                self.state.select(Some(0))
            }
        }
        Ok(())
    }
}
