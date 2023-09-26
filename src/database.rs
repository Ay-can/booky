pub mod models;
pub mod schema;

use crate::app::App;
use crate::database::models::*;
use crate::database::schema::books::dsl::books;
use crate::database::schema::books::*;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dirs_2::document_dir;

pub fn get_db_path() -> String {
    document_dir()
        .expect("Failed to find /Documents")
        .join("booky")
        .join("books.db")
        .display()
        .to_string()
}

pub fn establish_connection() -> SqliteConnection {
    // Sqlite will automatically create books.db if it does't exist
    let path = get_db_path();
    let mut connection =
        SqliteConnection::establish(&path).unwrap_or_else(|_| panic!("Error connecting to"));
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

    // Find a better way to do this...
    let title_pattern = format!("%{}%", book_info.title);
    let author_pattern = format!("%{}%", book_info.author);
    let genre_pattern = format!("%{}%", book_info.genre);
    let status_pattern = format!("%{}%", book_info.status);

    let results = books
        .select(Book::as_select())
        .filter(title.like(title_pattern))
        .filter(author.like(author_pattern))
        .filter(genre.like(genre_pattern))
        .filter(status.like(status_pattern))
        .filter(rating.ge(book_info.rating))
        .filter(start_date.ge(book_info.start_date))
        .filter(end_date.le(book_info.end_date))
        .load(connection)
        .expect("Failed to find books");
    results
}
