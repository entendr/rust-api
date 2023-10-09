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
use chrono::prelude::*;

// this

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

use crate::database::Database;
use crate::events::Event;

#[launch]
async fn rocket() -> _ {
  
  let db: Database = db!();

  // load dummy data
  // load_dummy_data(&db).await;
  
  rocket::build()
    .manage(db)
    .mount("/", routes![hello])
    .mount("/", routes![get_events])
    .mount("/", routes![put_event])
    .mount("/", routes![get_event])
}

///
/// Hello world
///
#[get("/")]
fn hello() -> Result<Json<String>, Status> {
  println!("Hello from rust and mongoDB");
  Ok(Json(String::from("Hello from rust and mongoDB")))
}

///
/// Get all events from the database
/// 
/// ## Parameters
/// 
/// None
/// 
/// ## Returns
/// 
/// All events (JSON) wrapped in a Result
/// 
#[get("/events")]
async fn get_events(db: &State<Database>) -> Result<Json<Vec<Event>>, Status> {
  let events = db.get_events().await.unwrap();
  println!("{:?}", events);
  Ok(Json(events))
}

///
/// 
/// Get a single event from the database
/// 
/// ## Parameters
/// 
/// - `id` - The id of the event to get
/// 
/// ## Returns
/// 
/// The event (JSON) wrapped in a Result
/// 
#[get("/events/<id>")]
async fn get_event(db: &State<Database>, id: String) -> Result<Json<Event>, Status> {
  let event: Event = db.get_event(id).await.unwrap_or_default();
  println!("{:?}", event);
  Ok(Json(event))
}

///
/// Insert a new event into the database
/// 
/// ## Parameters
/// 
/// - `event` - The event to insert
/// 
/// ## Returns
/// 
/// The id of the inserted event wrapped in a Result
///
#[put("/events", data = "<event>")]
async fn put_event(db: &State<Database>, event: Json<Event>) -> Result<Json<String>, Status> {
  let id: String = db.insert_event(event.into_inner()).await.unwrap();
  Ok(Json(id))
}

/// do stuff
async fn load_dummy_data(db: &Database) {
  // create 10 events and add them to db
  for i in 0..10 {
    let event = event!(
      format!("Event {}", i),
      format!("Description {}", i),

      // convert now time to mongodb bson time


      mongodb::bson::DateTime::from_system_time(std::time::SystemTime::now()),
      format!("Location {}", i),
      vec![format!("Participant {}", i)]
    );

    print!("inserting event: {:?}\n", event);
    let res = db.insert_event(event).await.unwrap();
    print!("inserted event: {:?}\n", res);
  }
}
