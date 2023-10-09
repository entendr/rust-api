/// main.rs
/// 
/// main entry point for the program
///

mod events;
mod database;
// use mini_redis::{client, Result};
use dotenv::dotenv;
use mongodb::{Client, options::ClientOptions, bson::doc};
use std::error::Error;
use rocket::{Rocket, Build, State};

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

use crate::database::Database;
use crate::events::Event;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
  Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[get("/events")]
async fn get_events(db: &State<Database>) -> Result<Json<Vec<Event>>, Status> {
  let events = db.get_events().await.unwrap();
  Ok(Json(events))
}

#[put("/events", data = "<event>")]
async fn put_event(db: &State<Database>, event: Json<Event>) -> Result<Json<String>, Status> {
  let id = db.insert_event(event.into_inner()).await.unwrap();
  Ok(Json(id))
}

#[launch]
async fn rocket() -> _ {
  let db = database::Database::init().await.unwrap();
  
  rocket::build()
    .manage(db)
    .mount("/", routes![hello])
    .mount("/", routes![get_events])
    .mount("/", routes![put_event])
}

