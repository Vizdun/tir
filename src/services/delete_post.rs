use crate::{
    db::{delete_post as delete_post_fn, establish_connection},
    verify::verify,
};

use chrono::Local;
use rocket::serde::{json::Json, Deserialize};

#[derive(Deserialize)]
pub struct DeletePostRequest {
    id: i32,
    timestamp: i64,
    signature: String,
}

#[post("/delete", data = "<rq>")]
pub fn delete_post(rq: Json<DeletePostRequest>) {
    if !verify(
        &[&rq.id.to_le_bytes()[..], &rq.timestamp.to_le_bytes()[..]].concat(),
        rq.signature.clone(),
    ) || Local::now().timestamp() - 60 > rq.timestamp
    {
        return;
    }

    let connection = establish_connection();

    delete_post_fn(&connection, rq.id);
}
