mod schema;
mod models;
mod posts;
mod engines;

use engines::{fetch_engines, create_engine, delete_engine};
use rocket::serde::json::{json, Value};
use rocket_diesel_shenanigans::establish_connection;

use posts::{create_post, delete_post, fetch_posts};

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
        .mount("/", routes![index, print_name, print_json])
        .mount("/posts", routes![fetch_posts, create_post, delete_post])
        .mount("/engines", routes![fetch_engines, create_engine, delete_engine])
        .register("/", catchers![not_found, internal_error, unprocessable_entity, default_catch])
}