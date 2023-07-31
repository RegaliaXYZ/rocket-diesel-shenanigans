use diesel::prelude::*;
use serde::Deserialize;
use crate::schema::posts;
use crate::schema::engines;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::engines)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Engine {
    pub id: i32,
    pub engine_name: String,
    pub tenant: String,
    pub engine_type: String,
    pub num_utterances: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = engines)]
pub struct NewEngine<'a> {
    pub engine_name: &'a str,
    pub tenant: &'a str,
    pub engine_type: &'a str,
    pub num_utterances: Option<i32>,
}