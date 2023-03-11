use std::env;
use aws_sdk_secretsmanager::{Client, Error};

pub async fn get_api_key() -> Result<String, Error> {
  // Check if env var with secret name is set
  let secret_name = match env::var("API_KEY_SECRET_NAME") {
      Ok(value) => value,
      Err(error) => panic!("API_KEY_SECRET_NAME is not set: {}", error),
  };

  // Configure AWS client
  let aws_config = aws_config::load_from_env().await;
  let sercrets_client = Client::new(&aws_config);

  // Fetch the secret value
  let response = sercrets_client.get_secret_value().secret_id(secret_name).send().await?;

  return Ok(String::from(response.secret_string().unwrap_or("No value!")));
}
