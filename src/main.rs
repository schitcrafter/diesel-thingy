use std::{env, sync::Arc};

use axum::{extract::FromRef, routing::get, Router};
use dotenvy::dotenv;
use tokio::sync::Mutex;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use self::models::*;
use diesel_async::{AsyncConnection, AsyncPgConnection};

pub mod models;
pub mod schema;
pub mod posts;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let db_connection = establish_connection().await;

    let state = PostsState {
        db:  Arc::new(Mutex::new(db_connection)),
    };


    let app = Router::new()
        .route("/hello", get(|| async { "Hello, World!" }))
        .nest("/posts", posts::get_routes())
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn establish_connection() -> AsyncPgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url).await
        .expect("Error connecting to db")
}

#[derive(FromRef, Clone)]
pub struct PostsState {
    pub db: Arc<Mutex<AsyncPgConnection>>,
}
