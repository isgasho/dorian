#[feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::*;

extern crate serde;
use serde::{Deserialize, Serialize};
use serde_json::*;

use std::error::Error;
use std::path::Path;

mod schema {
    table! {
        entries (id) {
            id -> Integer,
            tags -> Text,
            uploader -> Text,
        }
    }

    table! {
        taglinks (id) {
            id -> Integer,
            tag_id -> Integer,
            entry_id -> Integer,
        }
    }

    table! {
        tags (id) {
            id -> Integer,
            name -> Text,
        }
    }

    table! {
        users (id) {
            id -> Integer,
            name -> Text,
        }
    }

    allow_tables_to_appear_in_same_query!(entries, taglinks, tags, users);
}

mod db {
    extern crate serde;
    use serde::{Deserialize, Serialize};

    use super::schema::*;

    //
    // These Entity structs represent what actually exists in the database.
    //

    #[derive(Serialize, Deserialize, Identifiable, Queryable)]
    #[table_name = "users"]
    pub struct UserEntity {
        id: i32,
        name: String,
    }

    #[derive(Serialize, Deserialize, Identifiable, Queryable)]
    #[table_name = "entries"]
    pub struct EntryEntity {
        id: i32,
        tags: Vec<String>,
        uploader: String, // User.name
    }

    #[derive(Serialize, Deserialize, Identifiable, Queryable)]
    #[table_name = "tags"]
    pub struct TagEntity {
        id: i32,
        name: String,
    }

    #[derive(Serialize, Deserialize, Identifiable, Queryable)]
    #[table_name = "taglinks"]
    pub struct TaglinkEntity {
        id: i32,
        tag_id: i32,
        entry_id: i32,
    }

    //
    // The structs below represent the structs that will be dealt with in rust code.
    //

    #[derive(Insertable)]
    pub struct User {
        name: String,
    }

    #[derive(Insertable)]
    #[table_name = "entries"] // diesel thought this was "entrys", funny
    pub struct Entry {
        uploader: String, // User.name
    }

    #[derive(Insertable)]
    pub struct Tag {
        name: String,
    }

    #[derive(Insertable)]
    pub struct Taglink {
        tag_id: i32,
        entry_id: i32,
    }
}

use std::result::Result;

fn get_tags(conn: &SqliteConnection) -> Result<Vec<String>, Box<dyn Error>> {
    use schema::tags::dsl::*;

    let ts = tags.select(name).load::<String>(conn)?;

    Ok(ts)
}

fn get_entries(conn: &SqliteConnection) -> Result<(Vec<i32>, Vec<String>), Box<dyn Error>> {
    use schema::entries::dsl::*;

    let (ids, us): (Vec<i32>, Vec<String>) = entries
        .select((id, uploader))
        .load::<(i32, String)>(conn)?
        .iter()
        .cloned()
        .unzip();

    Ok((ids, us))
}

fn get_entry_tags(conn: &SqliteConnection, eid: i32)
/* -> Result<(String, Vec<String>), Box<dyn Error>> */
{
    use schema::taglinks::dsl::*;
    use schema::tags::dsl::*;

    /* BROKEN
        // search the tag_ids in taglinks for eid (the id of the entry we're looking up the tags for)
        let ts: Vec<String> = taglinks
            .filter(entry_id.eq(eid))
            .select(tag_id)
            .load::<i32>(conn)
            .expect("Could not load taglinks from db")
            .iter()
            .map(|tid|
        // search tags for the names of the tags associated with each tag_id
        tags.filter(tag_id.eq(tid)).select(name).load::<String>(conn).expect("Could not load tags from db"))
    BROKEN */
}

fn new_tag(conn: &SqliteConnection) /* -> Result<&'static str, &'static str> */
{
    /*
    let i = 0;
    match i {
        0 => Err("Error"),
        _ => Ok("OK"),
    }
    */
}

fn main() {
    use db::*;
    use schema::tags::dsl::*;

    let conn = SqliteConnection::establish("dorian.db").expect("error connecting to db");

    /* Broken
    let serialized = json!({ "tags": tags_in_db });
    println!("{}", serialized);
    */
}
