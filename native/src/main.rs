#[macro_use]
extern crate serde;

pub mod client;
pub mod user;

use client::InnerClient;
use std::{convert::Infallible, sync::Arc};
use warp::{reply::json, Filter, Reply};

async fn fetch_users(db: Arc<InnerClient>) -> Result<impl Reply, Infallible> {
    Ok(json(&db.users().await.unwrap()))
}

async fn fetch_big(db: Arc<InnerClient>) -> Result<impl Reply, Infallible> {
    Ok(json(&db.big_users().await.unwrap()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = Arc::new(InnerClient::new("file:../test.db").await?);

    let users_db = db.clone();
    let users = warp::path("users")
        .and(warp::any().map(move || users_db.clone()))
        .and_then(fetch_users);

    let big = warp::path("big")
        .and(warp::any().map(move || db.clone()))
        .and_then(fetch_big);

    let routes = warp::get().and(users.or(big));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
