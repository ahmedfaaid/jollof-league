use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorMessage(String);

impl ErrorMessage {
    pub fn new(message: &str) -> Self {
      ErrorMessage(message.to_string())
    }
}