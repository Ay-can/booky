use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use std::error;
use std::fs;
use std::path::Path;

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
    pub rating: f64,
    pub status: String,
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

    pub fn read_json(&mut self) -> Result<Vec<Book>, Box<dyn error::Error>> {
        let json_content = fs::read_to_string(JSON_PATH)?;
        let parsed: Vec<Book> = serde_json::from_str(&json_content)?;
        self.items = parsed.clone();
        Ok(parsed)
    }

    // Close your eyes, this is temp fix
    pub fn read_json_3(&mut self) -> Result<Vec<Book>, Box<dyn error::Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "booky") {
            let config_dir: &Path = proj_dirs.config_dir();
            let new_path = config_dir.join("books.json");
            let books: Vec<Book> = {
                let config_contents = fs::read_to_string(&new_path).expect("Failed to read");
                serde_json::from_str(&config_contents).unwrap()
            };
            self.items = books.clone();
            Ok(books)
        } else {
            let books: Vec<Book> = vec![];
            Ok(books)
        }
    }

    pub fn create_json(&mut self) {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "booky") {
            let config_dir: &Path = proj_dirs.config_dir();
            if !config_dir.exists() {
                fs::create_dir(config_dir);
                fs::write(config_dir.join("books.json"), "[]");
            }
        }
    }

    pub fn remove_json_at_index(&mut self) -> Result<(), Box<dyn error::Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "booky") {
            let config_dir: &Path = proj_dirs.config_dir();
            let new_path = config_dir.join("books.json");
            if let Some(selected) = self.state.selected() {
                self.items.remove(selected);
                fs::write(new_path, &serde_json::to_vec(&self.items)?)?;

                if selected > 0 {
                    self.state.select(Some(selected - 1))
                } else {
                    self.state.select(Some(0))
                }
            }
        }
        Ok(())
    }
}
