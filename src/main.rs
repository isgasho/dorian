use serde::{Deserialize, Serialize};
use serde_json::Result;
use uuid::Uuid;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct User {
    uuid: String, // uuid
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Image {
    uuid: String, // uuid
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    image: Image,
    user: User,
}

#[tokio::main]
async fn main() {
    let upload = warp::path!("upload").map(|| format!("{}", blob));
    warp::serve(upload).run(([0, 0, 0, 0], 3030)).await;
}
