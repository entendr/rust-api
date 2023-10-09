use dotenv::dotenv;
use rocket::futures::TryStreamExt;
use std::error::Error;
use mongodb::{
    bson::{extjson::de::Error as JSONError, oid::ObjectId, doc},
    options::ClientOptions,
    Client, Collection,
    error::Error as DatabaseError
};
use crate::events::Event;

pub struct Database {
  col: Collection<Event>,
}

impl Database {
  pub async fn init() -> Result<Self, Box<dyn Error>> {
    dotenv().ok();

    // load uri from env
    let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set");


    // connect to mongodb
    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;

    // get a handle to the database
    let db = client.database("events");

    // get a handle to the collection
    let col = db.collection("events");
    
    Ok(Database { col })
  }

  /// Insert a new event into the database
  /// 
  /// ## Parameters
  /// 
  /// * `event` - The event to insert
  /// 
  /// ## Returns
  /// 
  /// The id of the inserted event wrapped in a Result
  pub async fn insert_event(&self, event: Event) -> Result<String, DatabaseError> {
    let result = self.col.insert_one(event, None).await;
    match result {
      Ok(result) => Ok(result.inserted_id.as_object_id().unwrap().to_string()),
      Err(e) => Err(e),
    }
  }

  /// Get all events from the database
  /// 
  /// ## Parameters
  /// 
  /// None
  /// 
  /// ## Returns
  /// 
  /// A vector of events wrapped in a Result
  pub async fn get_events(&self) -> Result<Vec<Event>, JSONError> {
    let cursor = self
      .col
      .find(None, None)
      .await.ok()
      .expect("Error finding events");
    let events = match cursor.try_collect().await.ok() {
      Some(events) => events,
      None => Vec::new(),
    };
    Ok(events)
  }

  /// Get a single event with the given id
  /// 
  /// ## Parameters
  /// 
  /// * `id` - The id of the event to get
  ///   
  /// ## Returns
  /// 
  /// An event wrapped in a Result
  pub async fn get_event(&self, id: String) -> Result<Event, JSONError> {
    let oid = ObjectId::parse_str(&id).unwrap_or_default();
    let filter = doc! { "_id": oid };
    let event = self
      .col
      .find_one(filter, None)
      .await
      .ok()
      .expect("Error finding event");
    Ok(event.unwrap_or_default())
  }
}

// define macro to initialize db
#[macro_export]
macro_rules! db {
  () => {
    Database::init().await.unwrap()
  };
}
