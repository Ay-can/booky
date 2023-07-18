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
use dotenvy::dotenv;
use std::env;
use std::error::Error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
type DB = diesel::sqlite::Sqlite;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn establish_connection_2() -> SqliteConnection {
    let database_url = "./books.db";
    let mut connection =
        SqliteConnection::establish(database_url).unwrap_or_else(|_| panic!("Error connecting to"));
    run_migrations(&mut connection);
    connection
}

pub fn create_book(new_book: NewBook) -> Book {
    use crate::database::schema::books;
    let connection = &mut establish_connection_2();

    diesel::insert_into(books::table)
        .values(&new_book)
        .returning(Book::as_returning())
        .get_result(connection)
        .expect("Error saving new book")
}

// Do this without app parameter later
pub fn get_books(app: &mut App) -> Vec<Book> {
    let connection = &mut establish_connection_2();

    let results = books
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");
    app.items = results.clone();
    results
}

pub fn update_book(book_id: i32, update_book: NewBook) {
    let connection = &mut establish_connection_2();

    diesel::update(books.find(book_id))
        .set(update_book)
        .execute(connection)
        .expect("Error updating book");
}

pub fn delete_book(app: &mut App) {
    let connection = &mut establish_connection_2();
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
