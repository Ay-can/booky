use super::schema::books;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, AsChangeset, Debug)]
#[diesel(table_name = crate::database::schema::books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub rating: i32,
    pub status: String,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}

#[derive(Insertable, AsChangeset, Debug)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub rating: i32,
    pub status: String,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
}
