use diesel::prelude::*

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id usize,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub rating: f64,
}
