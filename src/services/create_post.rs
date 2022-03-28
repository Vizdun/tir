use crate::{
    db::{create_post as create_post_fn, establish_connection},
    verify::verify,
};

use rocket::serde::{json::Json, Deserialize};

use chrono::Local;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    title: String,
    body: String,
    timestamp: i64,
    signature: String,
}

#[post("/create", data = "<rq>")]
pub fn create_post(rq: Json<CreatePostRequest>) {
    if !verify(
        &[
            rq.title.as_bytes(),
            rq.body.as_bytes(),
            &rq.timestamp.to_le_bytes(),
        ]
        .concat(),
        rq.signature.clone(),
    ) || Local::now().timestamp() - 60 > rq.timestamp
    {
        return;
    }

    let connection = establish_connection();
    
    create_post_fn(&connection, &rq.title, &rq.body);
}
