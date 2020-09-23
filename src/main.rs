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

    for tag in backend::get_tags(&pool).await?.iter() {
        println!("got: {}", tag.name)
    }

    // select all tags for entry 1
    println!("entry 1:");

    for tag in backend::get_entry_tags(&pool, 1).await?.iter() {
        println!("got: {}", tag.name)
    }

    Ok(())
}
