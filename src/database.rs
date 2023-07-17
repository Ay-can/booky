pub mod models;
pub mod schema;

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

pub fn get_books() {
    use crate::database::schema::books;

    let connection = &mut establish_connection();
    let results = books
        .limit(5)
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");

    println!("Displaying {} books", results.len());

    for book in results {
        println!("{}", book.title);
    }
}

pub fn create_book(
    conn: &mut SqliteConnection,
    title: String,
    author: String,
    genre: String,
    rating: i32,
    status: String,
) -> Book {
    use crate::database::schema::books;

    let new_book = NewBook {
        title,
        author,
        genre,
        rating,
        status,
    };

    diesel::insert_into(books::table)
        .values(&new_book)
        .returning(Book::as_returning())
        .get_result(conn)
        .expect("Error saving new book")
}
