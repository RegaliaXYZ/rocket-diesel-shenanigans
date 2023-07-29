use rocket::serde::json::{json, Value};


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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, print_name, print_json])
        .register("/", catchers![not_found, internal_error])
}