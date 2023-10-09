/// Events
/// 

use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub name: String,
  pub description: String,
  pub time: DateTime,
  pub location: String,
  pub participants: Vec<String>,
}

impl Default for Event {
  fn default() -> Self {
    Event {
      id: None,
      name: String::from(""),
      description: String::from(""),
      time: DateTime::now(),
      location: String::from(""),
      participants: Vec::new(),
    }
  }
}

impl Event {
  pub fn new(name: String, description: String, time: DateTime, location: String, participants: Vec<String>) -> Self {
    Event {
      id: None,
      name,
      description,
      time,
      location,
      participants,
    }
  }
}

// macro to create an event
#[macro_export]
macro_rules! event {
  ($name:expr, $description:expr, $time:expr, $location:expr, $participants:expr) => {
    Event::new($name, $description, $time, $location, $participants)
  };
}
