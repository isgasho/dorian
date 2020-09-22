extern crate dotenv;
extern crate eyre;
extern crate serde;

use async_std;
use dotenv::dotenv;

use std::env;

use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use serde_json::*;
use sqlx::postgres::PgPool;
use sqlx::Done;

use std::error::Error;

mod db {
    extern crate serde;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct User {
        pub id: Option<i32>,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct Entry {
        pub id: Option<i32>,
        pub uploader: String, // User.name
    }

    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct Tag {
        pub id: Option<i32>,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct Tagmap {
        pub id: Option<i32>,
        pub tag_id: i32,
        pub entry_id: i32,
    }
}

// select all the tags
async fn get_tags(pool: &PgPool) -> eyre::Result<Vec<db::Tag>> {
    let tags = sqlx::query_as("SELECT * FROM tags").fetch_all(pool).await?;

    Ok(tags)
}

// select all the entries
async fn get_entries(pool: &PgPool) -> eyre::Result<Vec<db::Entry>> {
    let entries = sqlx::query_as("SELECT * FROM entries")
        .fetch_all(pool)
        .await?;

    Ok(entries)
}

// select all the tags that are associated with the given entry_id in the tagmap
async fn get_entry_tags(pool: &PgPool, eid: i32) -> eyre::Result<Vec<db::Tag>> {
    let tags = sqlx::query_as(
        "
        SELECT tags.name FROM tags
            INNER JOIN tagmap  ON tagmap.tag_id = tags.id
            INNER JOIN entries ON tagmap.entry_id = entries.id
        WHERE entries.id = $1;
        ",
    )
    .bind(&eid)
    .fetch_all(pool)
    .await?;

    Ok(tags)
}

// add a vector of  new tags to the db
fn new_tags(pool: &PgPool, ts: Vec<db::Tag>) -> eyre::Result<()> {
    Ok(())
}

// Create a new Entry
fn new_entry(pool: &PgPool, entry: db::Entry) -> eyre::Result<()> {
    Ok(())
}

fn tag_entry(pool: &PgPool, ts: Vec<db::Tag>, en: db::Entry) -> eyre::Result<()> {
    Ok(())
}

#[async_std::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    for tag in get_tags(&pool).await?.iter() {
        println!("got: {}", tag.name)
    }

    // select all tags for entry 1
    println!("entry 1:");

    for tag in get_entry_tags(&pool, 1).await?.iter() {
        println!("got: {}", tag.name)
    }

    Ok(())
}
