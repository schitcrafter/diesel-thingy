use diesel::prelude::*;
use validator::Validate;
use crate::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, AsChangeset, Deserialize, Validate)]
#[diesel(table_name = posts)]
pub struct NewPost {
    #[validate(length(min = 10, max = 100))]
    pub title: String,
    pub body: String,
}