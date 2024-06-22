use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, routing::{get, post}, Json, Router};
use axum_valid::Valid;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use tokio::sync::Mutex;

use crate::{NewPost, Post, PostsState};


pub fn get_routes() -> Router<PostsState> {
    Router::new()
        .route("/", get(get_all_posts).post(add_post))
        .route("/:id/publish", post(publish_post))
        .route("/:id", post(update_post))
}

pub async fn update_post(
    State(db): State<Arc<Mutex<AsyncPgConnection>>>,
    Path(post_id): Path<i32>,
    Valid(Json(update_post_body)): Valid<Json<NewPost>>
) -> Result<Json<Post>, (StatusCode, &'static str)> {
    use crate::schema::posts::dsl::*;
    
    let mut db_handle = db.lock().await;

    let post = diesel::update(posts.find(post_id))
        .set(&update_post_body)
        .get_result(&mut *db_handle).await
        .optional()
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database connection failed"))?;

    post.ok_or((StatusCode::BAD_REQUEST, "Could not find post"))
        .map(Json)
}

pub async fn publish_post(
    State(state): State<PostsState>,
    Path(post_id): Path<i32>,
) -> Result<Json<Post>, (StatusCode, &'static str)> {
    use crate::schema::posts::dsl::*;
    let mut db_handle = state.db.lock().await;


    let maybe_post: Option<Post> = diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .get_result(&mut *db_handle).await
        .optional()
        .map_err(|_err| (StatusCode::INTERNAL_SERVER_ERROR, "Database connection failed"))?;

    maybe_post
        .ok_or((StatusCode::BAD_REQUEST, "Could not find post"))
        .map(Json)
}

pub async fn get_all_posts(State(state): State<PostsState>) -> Result<Json<Vec<Post>>, StatusCode> {
    use crate::schema::posts::dsl::*;

    let mut db_handle = state.db.lock().await;
    
    posts.select(Post::as_select())
        .limit(100)
        .load(&mut *db_handle).await
        .map(Json)
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn add_post(
    State(state): State<PostsState>,
    Valid(Json(new_post_body)): Valid<Json<NewPost>>
) -> Result<Json<Post>, StatusCode> {
    use crate::schema::posts::dsl::*;

    let mut db_handle = state.db.lock().await;

    diesel::insert_into(posts)
        .values(&new_post_body)
        .get_result(&mut *db_handle).await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
