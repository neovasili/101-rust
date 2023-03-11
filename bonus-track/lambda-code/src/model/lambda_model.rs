use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
  pub body: String,
}

#[derive(Debug, Serialize)]
pub struct FailureResponse {
  pub body: String,
}

// Implement Display for the Failure response so that we can then implement Error.
impl std::fmt::Display for FailureResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.body)
  }
}

pub type Response = Result<SuccessResponse, FailureResponse>;

#[derive(Debug, Deserialize)]
pub struct RequestPayload {
  pub prompt: String,
}
