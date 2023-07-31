
use rocket::serde::json::{json, Value, Json};

use crate::{establish_connection, models};
use diesel::prelude::*;
use serde::Deserialize;
use crate::models::Engine;


#[get("/")]
pub fn fetch_engines() -> Value {
    use crate::schema::engines::dsl::*;

    let connection = &mut establish_connection();
    let results = engines
        .limit(5)
        .load::<Engine>(connection)
        .expect("Error loading engines");

    let engine_names = results.iter().map(|engine| engine.engine_name.clone()).collect::<Vec<String>>();
    json!({
        "status": "success",
        "data": engine_names,
    })
}

#[post("/", data = "<new_engine>")]
pub fn create_engine(new_engine: Json<models::NewEngine>) -> Value {
    use crate::schema::engines::dsl::*;

    let connection = &mut establish_connection();
    let engine = diesel::insert_into(engines)
        .values(new_engine.into_inner())
        .get_result::<Engine>(connection)
        .expect("Error saving new engine");

    json!({
        "status": "success",
        "data": engine.engine_name,
    })
}

#[derive(Deserialize)]
pub struct DeleteEngine {
    name: String,
}

#[delete("/", data = "<delete_engine>")]
pub fn delete_engine(delete_engine: Json<DeleteEngine>) -> Value {
    use crate::schema::engines::dsl::*;

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(engines.filter(engine_name.eq(delete_engine.into_inner().name)))
        .execute(connection)
        .expect("Error deleting engine");

    json!({
        "status": "success",
        "data": num_deleted,
    })
}
