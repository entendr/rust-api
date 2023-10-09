use std::env;
extern crate dotenv;
use dotenv::dotenv;
use std::error::Error;
use mongodb::{
    bson::{extjson::de::Error as DatabaseError},
    results::{ InsertOneResult},
    options::ClientOptions,
    Client, Collection,
};
use crate::events::Event;

pub struct Database {
  col: Collection<Event>,
}

impl Database {
  pub async fn init() -> Self {
    dotenv().ok();
      //   dotenv().ok();

    // load uri from env
    let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set");


    // connect to mongodb
    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;

    // get a handle to the database
    let db = client.database("events");

    // get a handle to the collection
    let col = db.collection("events");
    
    Database { col }
  }

  pub async fn get_events(&self) -> Result<Vec<Event>, DatabaseError> {
    let cursor = self
      .col
      .find(None, None)
      .await
      .ok()
      .expect("Error finding events");
    let events: Vec<Event> = cursor.map(|doc| doc.unwrap()).collect::<Vec<Event>>();
    Ok(events)
  }

  pub async fn get_event(&self) -> Result<Event, DatabaseError> {
    
  }
}
