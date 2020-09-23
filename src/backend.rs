// extern crate serde;
// use serde::{Deserialize, Serialize};
// use serde_json::*;

use eyre;
use sqlx;
use sqlx::postgres::PgPool;
use sqlx::Done;

mod db {
    extern crate serde;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct User {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct Entry {
        pub id: i32,
        pub uploader: String, // User.name
    }

    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct Tag {
        pub id: i32,
        pub name: String,
    }

    #[derive(Serialize, Deserialize, sqlx::FromRow)]
    pub struct Tagmap {
        pub id: i32,
        pub tag_id: i32,
        pub entry_id: i32,
    }
}

// select all the tags
pub async fn get_tags(pool: &PgPool) -> eyre::Result<Vec<db::Tag>> {
    let tags = sqlx::query_as("SELECT * FROM tags").fetch_all(pool).await?;

    Ok(tags)
}

// select all the entries
pub async fn get_entries(pool: &PgPool) -> eyre::Result<Vec<db::Entry>> {
    let entries = sqlx::query_as!(db::Entry, "SELECT * FROM entries")
        .fetch_all(pool)
        .await?;

    Ok(entries)
}

// select all the tags that are associated with the given entry_id in the tagmap
pub async fn get_entry_tags(pool: &PgPool, eid: i32) -> eyre::Result<Vec<db::Tag>> {
    let tags = sqlx::query_as!(
        db::Tag,
        "SELECT tags.id, tags.name FROM tags
            INNER JOIN tagmap  ON tagmap.tag_id = tags.id
            INNER JOIN entries ON tagmap.entry_id = entries.id
        WHERE entries.id = $1;",
        eid
    )
    .fetch_all(pool)
    .await?;

    Ok(tags)
}

// add a vector of new tags to the db
pub async fn new_tags(pool: &PgPool, ts: Vec<String>) -> eyre::Result<u64> {
    // TODO: figure out how to return IDs better
    let ra = sqlx::query!(
        "
        INSERT INTO tags (name)
        SELECT * FROM UNNEST($1::text[])
        ",
        &ts.as_slice(),
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(ra)
}

// Create a new entry
pub async fn new_entry(pool: &PgPool, es: Vec<String>) -> eyre::Result<u64> {
    let ra = sqlx::query!(
        "
        INSERT INTO entries (uploader)
        SELECT * FROM UNNEST($1::text[])
        ",
        &es.as_slice(),
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(ra)
}

// add tags to an entry
pub async fn tag_entry(pool: &PgPool, ts: Vec<db::Tag>, en: db::Entry) -> eyre::Result<u64> {
    let tag_ids: Vec<i32> = ts.iter().map(|t| t.id).collect();

    let ra = sqlx::query!(
        "
        INSERT INTO tagmap (entry_id, tag_id)
        (SELECT $1, * FROM UNNEST($2::integer[]));
        ",
        en.id,
        tag_ids.as_slice(),
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(ra)
}
