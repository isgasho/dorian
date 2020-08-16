use std::error::Error;
use std::path::Path;
use uuid::Uuid;

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

impl User {
    fn new(n: String) -> User {
        User {
            name: n,
            id: Uuid::new_v4().to_u128_le(),
        }
    }
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "entries"] // diesel thought this was "entrys", funny
struct Entry {
    id: u128,
    tags: Vec<String>,
    uploader: String, // User.name
}

impl Entry {
    fn new(u: User, ts: Vec<String>) -> Entry {
        Entry {
            id: Uuid::new_v4().to_u128_le(),
            uploader: u.name,
            tags: ts,
        }
    }
}

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
struct Tag {
    id: u128,
    name: String,
}

impl Tag {
    fn new(n: String) -> Tag {
        Tag {
            id: Uuid::new_v4().to_u128_le(),
            name: n,
        }
    }
}
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
struct Taglink {
    id: u128,
    tag_id: u128,
    entry_id: u128,
}

impl Taglink {
    fn new(tag: Tag, entry: Entry) -> Taglink {
        Taglink {
            id: Uuid::new_v4().to_u128_le(),
            tag_id: tag.id,
            entry_id: entry.id,
        }
    }
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
    let qr = tags
        .select(name)
        .load::<String>(&connection)
        .expect("Error loading tags");

    for tag in qr.iter() {
        println!("{}", tag);
    }

    let health = warp::path!("health").map(|| "200 OK");
    warp::serve(health).run(([0, 0, 0, 0], 3030)).await;
}
