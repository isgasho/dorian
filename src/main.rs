extern crate dotenv;
extern crate sqlx;

use std::env;

use async_std;
use dotenv::dotenv;
use sqlx::postgres::PgPool;

mod backend;

#[async_std::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let tags: Vec<backend::db::Tag> = backend::get_tags(&pool).await?;

    for tag in &tags {
        println!("got: {}", tag.name)
    }

    // TODO: figure out how im gonna handle errors. if you run dorian twice
    // it errors out because these tags already exist
    backend::new_tags(
        &pool,
        vec![
            String::from("wm:dwm"),
            String::from("wm:awesome"),
            String::from("wm:fvwm"),
            String::from("term:st"),
        ],
    )
    .await?;

    // select all tags for entry 1
    println!("entry 1:");

    backend::new_entry(&pool, vec![String::from("kori")]).await?;

    for tag in backend::get_entry_tags(&pool, 1).await?.iter() {
        println!("got: {}", tag.name)
    }

    println!("entry 1 mod:");

    backend::tag_entry(
        &pool,
        tags,
        backend::db::Entry {
            id: 1,
            uploader: String::from("kori"),
        },
    )
    .await?;

    for tag in backend::get_entry_tags(&pool, 1).await?.iter() {
        println!("got: {}", tag.name)
    }

    Ok(())
}
