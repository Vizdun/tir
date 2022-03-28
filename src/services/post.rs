use diesel::prelude::*;
use pulldown_cmark::{Options, Parser, html::push_html};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::{db::establish_connection, models::Post};

#[derive(Serialize)]
struct RenderedPost {
    title: String,
    body: String,
}

#[get("/post/<post_id>")]
pub fn post(post_id: u32) -> Option<Template> {
    use crate::schema::posts::dsl::*;

    let connection = establish_connection();

    let post = posts
        .limit(1)
        .filter(id.eq(post_id as i32))
        .load::<Post>(&connection).unwrap();

    let post = post.first()?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    let md_parse = Parser::new_ext(&post.body, options);
    let mut unsafe_html = String::new();
    push_html(&mut unsafe_html, md_parse);

    unsafe_html = unsafe_html.replace(
        "<table>",
        r#"<table class="table table-striped table-hover">"#,
    );

    let context = RenderedPost {
        title: post.title.clone(),
        body: unsafe_html,
    };

    Some(Template::render("post", &context))
}