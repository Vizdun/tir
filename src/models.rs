use diesel::Queryable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub pubdate: i64,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub pubdate: i64
}