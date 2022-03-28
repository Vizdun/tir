use crate::models::{NewPost, Post};
use chrono::Local;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
        pubdate: Local::now().timestamp(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn delete_post(conn: &PgConnection, del_id: i32) {
    use crate::schema::posts::dsl::*;

    diesel::delete(posts.filter(id.eq(del_id)))
        .execute(conn)
        .expect("Error deleting post");
}
