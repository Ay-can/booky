use crate::app::{App, Book};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::error;
use std::fs;
use std::path::Path;

pub fn write_json(app: &mut App, book: Book) -> Result<(), Box<dyn error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "booky") {
        let config_dir: &Path = proj_dirs.config_dir();
        let new_path = config_dir.join("books.json");
        app.items.push(book);
        fs::write(new_path, &serde_json::to_vec(&app.items)?)?;
    }
    Ok(())
}
