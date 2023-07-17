/*
use crate::app::{App, Book};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error;
use std::fs;
use std::path::Path;

// Need to refeactor all the (if let Some), because I don't want duplicate code
pub fn write_json(app: &mut App, book: Book) -> Result<(), Box<dyn error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "booky") {
        let config_dir: &Path = proj_dirs.config_dir();
        let new_path = config_dir.join("books.json");
        app.items.push(book);
        fs::write(new_path, &serde_json::to_vec(&app.items)?)?;
    }
    Ok(())
}

// temp fix
pub fn read_json(app: &mut App) -> Result<Vec<Book>, Box<dyn error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "booky") {
        let config_dir: &Path = proj_dirs.config_dir();
        let new_path = config_dir.join("books.json");
        let books: Vec<Book> = {
            let config_contents = fs::read_to_string(&new_path).expect("Failed to read");

            serde_json::from_str(&config_contents).unwrap()
        };
        app.items = books.clone();
        Ok(books)
    } else {
        let books: Vec<Book> = vec![];
        Ok(books)
    }
}

pub fn create_json() {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "booky") {
        let config_dir: &Path = proj_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir(config_dir);
            fs::write(config_dir.join("books.json"), "[]");
        }
    }
}

pub fn remove_json_at_index(app: &mut App) -> Result<(), Box<dyn error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "booky") {
        let config_dir: &Path = proj_dirs.config_dir();
        let new_path = config_dir.join("books.json");
        if let Some(selected) = app.state.selected() {
            app.items.remove(selected);
            fs::write(new_path, &serde_json::to_vec(&app.items)?)?;

            if selected > 0 {
                app.state.select(Some(selected - 1));
            } else {
                app.state.select(Some(0))
            }
        }
    }
    Ok(())
}
*/
