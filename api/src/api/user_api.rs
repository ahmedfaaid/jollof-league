use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data="<new_user>")]
pub fn create_user(db: &State<MongoRepo>, new_user: Json<User>) -> Result<Json<InsertOneResult>, Status> {
  let data = User {
    _id: None,
    first_name: new_user.first_name.to_owned(),
    last_name: new_user.last_name.to_owned(),
    email: new_user.email.to_owned(),
    phone_number: new_user.email.to_owned()
  };
  let user_detail = db.create_user(data);
  match user_detail {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err(Status::InternalServerError)
  }
}

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<User>, Status> {
  let id = path;
  if id.is_empty() {
    return Err(Status::BadRequest);
  };
  let user_detail = db.get_user(&id);
  match user_detail {
    Ok(user) => Ok(Json(user)),
    Err(_) => Err(Status::InternalServerError)
  }
}

#[get("/user")]
pub fn get_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
  let users = db.get_users();
  match users {
    Ok(users) => Ok(Json(users)),
    Err(_) => Err(Status::InternalServerError)
  }
}