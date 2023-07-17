pub mod models;
pub mod schema;

use crate::app::App;
use crate::database::models::*;
use crate::database::schema::books::dsl::books;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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
    use crate::database::schema::books;

    let connection = &mut establish_connection();

    let results = books
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");
    app.items = results.clone();
    results
}
