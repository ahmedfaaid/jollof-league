use crate::{models::{user_model::User, error_model::ErrorMessage}, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data="<new_user>")]
pub fn create_user(db: &State<MongoRepo>, new_user: Json<User>) -> Result<Json<User>, Json<ErrorMessage>> {
  let data = User {
    _id: None,
    first_name: new_user.first_name.to_owned(),
    last_name: new_user.last_name.to_owned(),
    email: new_user.email.to_owned(),
    phone_number: new_user.phone_number.to_owned()
  };

  match db.create_user(data) {
    Ok(user) => Ok(user),
    Err(_) => Err(Json(ErrorMessage::new("Error creating user")))
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