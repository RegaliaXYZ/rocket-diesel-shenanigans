mod schema;
mod models;

use rocket::serde::json::{json, Value, Json};
use rocket_diesel_shenanigans::establish_connection;
use diesel::prelude::*;
use serde::Deserialize;
use crate::models::Post;


#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn print_name(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/json/<name>")]
fn print_json(name: &str) -> Value {
    json!({
        "name": name,
    })
}

#[get("/posts")]
fn fetch_posts() -> Value {
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

#[post("/posts", data = "<new_post>")]
fn create_post(new_post: Json<models::NewPost>) -> Value {
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
struct DeletePost {
    title: String,
}

#[delete("/posts", data = "<delete_post>")]
fn delete_post(delete_post: Json<DeletePost>) -> Value {
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


#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[catch(500)]
fn internal_error() -> Value {
    json!({
        "status": "error",
        "reason": "Internal server error."
    })
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!({
        "status": "error",
        "reason": "Unprocessable entity."
    })
}

#[catch(default)]
fn default_catch() -> Value {
    json!({
        "status": "error",
        "reason": "Unknown error."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, print_name, print_json, fetch_posts, create_post, delete_post])
        .register("/", catchers![not_found, internal_error, unprocessable_entity, default_catch])
}