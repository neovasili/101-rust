use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMessage {
  pub role: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
  pub model: String,
  pub messages: [RequestMessage; 1],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
  pub role: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseChoice {
  pub message: ResponseMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
  pub choices: [ResponseChoice; 1],
}
