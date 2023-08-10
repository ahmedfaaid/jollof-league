use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
  bson::{extjson::de::Error, oid::ObjectId, doc},
  sync::{Client, Collection}
};
use rocket::serde::json::Json;
use crate::models::user_model::User;

pub struct MongoRepo {
  col: Collection<User>
}

impl MongoRepo {
  pub fn init() -> Self {
    dotenv().ok();
    let uri = match env::var("MONGO_URI") {
      Ok(v) => v.to_string(),
      Err(_) => format!("Error loading env variable")
    };
    let client = Client::with_uri_str(uri).unwrap();
    let db = client.database("jollof_league");
    let col: Collection<User> = db.collection("User");
    MongoRepo { col }
  }

  pub fn create_user(&self, new_user: User) -> Result<Json<User>, Error> {
    let new_doc = User {
      _id: None,
      first_name: new_user.first_name,
      last_name: new_user.last_name,
      email: new_user.email,
      phone_number: new_user.phone_number
    };
    let inserted_user = self.col.insert_one(&new_doc, None).ok().expect("Error creating user");
    let new_user = User {
      _id: inserted_user.inserted_id.as_object_id(),
      first_name: new_doc.first_name,
      last_name: new_doc.last_name,
      email: new_doc.email,
      phone_number: new_doc.phone_number
    };
    Ok(Json(new_user))
  }

  pub fn get_user(&self, _id: &String) -> Result<User, Error> {
    let obj_id = ObjectId::parse_str(_id).unwrap();
    let filter = doc! {"_id": obj_id};
    let user_detail = self.col.find_one(filter, None).ok().expect("Error getting user's details");
    Ok(user_detail.unwrap())
  }

  pub fn get_users(&self) -> Result<Vec<User>, Error> {
    let cursors = self.col.find(None, None).ok().expect("Error getting list of users");
    let users = cursors.map(|doc| doc.unwrap()).collect();
    Ok(users)
  }
}