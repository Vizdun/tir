use chrono::NaiveDateTime;
use diesel::RunQueryDsl;
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{db::establish_connection, models::Post};

#[derive(Serialize)]
struct PostGroup {
    date: String,
    posts: Vec<crate::models::Post>,
}

#[derive(Serialize)]
struct Posts {
    post_group: Vec<PostGroup>,
}

#[get("/")]
pub fn index() -> Template {
    use crate::schema::posts::dsl::*;

    let connection = establish_connection();
    let mut results = posts
        .load::<Post>(&connection)
        .expect("Error loading posts")
        .into_iter();

    let mut grouped: Vec<Vec<crate::models::Post>> = vec![];

    grouped.push(vec![results.next().unwrap()]);

    while let Some(res) = results.next() {
        if res.pubdate / 86400 == grouped.last().unwrap().first().unwrap().pubdate / 86400 {
            grouped.last_mut().unwrap().push(res);
        } else {
            grouped.push(vec![res]);
        }
    }

    let context = Posts {
        post_group: grouped
            .into_iter()
            .map(|x| PostGroup {
                date: NaiveDateTime::from_timestamp(x.first().unwrap().pubdate, 0)
                    .format("%A %e of %B %Y")
                    .to_string(),
                posts: x,
            })
            .rev()
            .collect(),
    };
    Template::render("index", &context)
}
