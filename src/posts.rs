
use rocket::serde::json::{json, Value, Json};

use crate::{establish_connection, models};
use diesel::prelude::*;
use serde::Deserialize;
use crate::models::Post;


#[get("/")]
pub fn fetch_posts() -> Value {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    let titles = results.iter().map(|post| post.title.clone()).collect::<Vec<String>>();
    json!({
        "status": "success",
        "data": titles,
    })
}

#[post("/", data = "<new_post>")]
pub fn create_post(new_post: Json<models::NewPost>) -> Value {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let post = diesel::insert_into(posts)
        .values(new_post.into_inner())
        .get_result::<Post>(connection)
        .expect("Error saving new post");

    json!({
        "status": "success",
        "data": post.title,
    })
}

#[derive(Deserialize)]
pub struct DeletePost {
    title: String,
}

#[delete("/", data = "<delete_post>")]
pub fn delete_post(delete_post: Json<DeletePost>) -> Value {
    use crate::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.eq(delete_post.into_inner().title)))
        .execute(connection)
        .expect("Error deleting post");

    json!({
        "status": "success",
        "data": num_deleted,
    })
}
