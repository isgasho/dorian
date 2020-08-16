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
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "entries"] // diesel thought this was "entrys", funny
struct Entry {
    id: i32,
    tags: Vec<String>,
    uploader: String, // User.name
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
struct Tag {
    id: i32,
    name: String,
}
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
struct Taglink {
    id: i32,
    tag_id: i32,
    entry_id: i32,
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

    let (i, n): (Vec<_>, Vec<_>) = tags
        .select((id, name))
        .load::<(i32, String)>(&connection)
        .expect("Error loading tags")
        .iter()
        .cloned()
        .unzip();

    println!("{}", n.join(" "));

    let t = warp::path!("tags").map(|| "");
    warp::serve(t).run(([0, 0, 0, 0], 3030)).await;
}
