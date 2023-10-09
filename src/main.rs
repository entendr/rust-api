/// main.rs
/// 
/// main entry point for the program
///

mod events;
// use mini_redis::{client, Result};
use dotenv::dotenv;
use mongodb::{Client, options::ClientOptions, bson::doc};
use std::error::Error;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
  Ok(Json(String::from("Hello from rust and mongoDB")))
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![hello])
}


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {

//   // load env vars from .env file
//   dotenv().ok();
  
//   // load uri from env
//   let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set");


//   // connect to mongodb
//   let client_options = ClientOptions::parse(&uri).await?;
//   let client = Client::with_options(client_options)?;

//   // get a handle to the database
//   let db = client.database("test");

//   // get a handle to the collection
//   let coll = db.collection("test");

//   // insert some documents
//   coll.insert_one(doc! { "x": 1 }, None).await?;

//   // return an option
//   Ok(())
// }
