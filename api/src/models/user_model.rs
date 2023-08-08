use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub _id: Option<ObjectId>,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub phone_number: String
}