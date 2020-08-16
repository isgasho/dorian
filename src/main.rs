use std::path::Path;

extern crate serde;
use serde::{Deserialize, Serialize};

extern crate warp;
use warp::Filter;

#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::*;

mod schema {
    diesel::table! {
        users {
            id -> Integer,
            name -> Text,
        }
    }
    diesel::table! {
        entries {
            id -> Integer,
            title -> Text,
            uploader -> Text,
        }
    }
    diesel::table! {
        tags {
            id -> Integer,
            name -> Text,
        }
    }
    diesel::table! {
        taglinks {
            id -> Integer,
            tag_id -> Integer,
            entry_id -> Integer,
        }
    }
}

use schema::*;

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
struct User {
    id: u128,
    name: String,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "entries"] // diesel thought this was "entrys", funny
struct Entry {
    id: u128,
    tags: Vec<String>,
    uploader: String, // User.name
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
struct Tag {
    id: u128,
    name: String,
}
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
struct Taglink {
    id: u128,
    tag_id: u128,
    entry_id: u128,
}

enum StorageError {
    FileNotFound,
    PermissionDenied,
    NotEnoughSpace,
}

// we're gonna use this to abstract the storage backend away. eventually other types
// of storage might be used and this is not super complex so this is fine
trait Store {
    fn save(&mut self) -> Result<u8, StorageError>;
    fn load<P>(path: P) -> Result<Vec<u8>, StorageError>
    where
        P: AsRef<Path>;
}

#[tokio::main]
async fn main() {
    use schema::tags::dsl::*;

    let connection = SqliteConnection::establish("dorian.db").expect("error connecting to db");

    let ts = tags
        .select(name)
        .load::<String>(&connection)
        .expect("Error loading tags")
        .join(" ");

    let t = warp::path!("tags").map(move || format!("{}", ts));

    warp::serve(t).run(([0, 0, 0, 0], 3030)).await;
}
