pub mod models;
pub mod schema;

use crate::app::App;
use crate::database::models::*;
use crate::database::schema::books::dsl::books;
use crate::database::schema::books::*;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dirs_2::document_dir;

use std::error::Error;
use std::fs;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

// Create migrations at runtime instead if using diesel cli
fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn establish_connection() -> SqliteConnection {
    let booky_dir = document_dir()
        .expect("Failed to create booky directory in /Documents")
        .join("booky");

    if !booky_dir.exists() {
        fs::create_dir(booky_dir);
    }

    let document_path = document_dir()
        .expect("Failed to find /Documents")
        .join("booky")
        .join("books.db");
    let document_path = document_path.display().to_string();

    // Sqlite will automatically create books.db if it does't exist
    let mut connection = SqliteConnection::establish(&document_path)
        .unwrap_or_else(|_| panic!("Error connecting to"));

    run_migrations(&mut connection);
    connection
}

pub fn create_book(new_book: NewBook) -> Book {
    use crate::database::schema::books;
    let connection = &mut establish_connection();

    diesel::insert_into(books::table)
        .values(&new_book)
        .returning(Book::as_returning())
        .get_result(connection)
        .expect("Error saving new book")
}

// Do this without app parameter later
pub fn get_books(app: &mut App) -> Vec<Book> {
    let connection = &mut establish_connection();

    let results = books
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");
    app.items = results.clone();
    results
}

pub fn update_book(book_id: i32, update_book: NewBook) {
    let connection = &mut establish_connection();

    diesel::update(books.find(book_id))
        .set(update_book)
        .execute(connection)
        .expect("Error updating book");
}

pub fn delete_book(app: &mut App) {
    let connection = &mut establish_connection();
    if let Some(selected) = app.state.selected() {
        let current_id = app.items.get(selected).unwrap().id;
        app.items.remove(selected);
        diesel::delete(books.filter(id.eq(current_id)))
            .execute(connection)
            .expect("Failed to delete book");

        if selected > 1 {
            app.state.select(Some(selected - 1))
        } else {
            app.state.select(Some(0))
        }
    }
}

pub fn search_book(book_info: NewBook) -> Vec<Book> {
    let connection = &mut establish_connection();

    // TEMP
    let pattern = format!("%{}%", book_info.title);
    let test_1 = format!("%{}%", book_info.author);
    let test_2 = format!("%{}%", book_info.genre);
    let test_4 = format!("%{}%", book_info.status);
    let results = books
        .select(Book::as_select())
        .filter(title.like(pattern))
        .filter(author.like(test_1))
        .filter(genre.like(test_2))
        .filter(status.like(test_4))
        .filter(rating.eq(book_info.rating))
        .load(connection)
        .expect("Failed to find books");
    results
}
