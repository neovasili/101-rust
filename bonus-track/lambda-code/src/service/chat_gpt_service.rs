use reqwest::Error;

#[path = "../model/chat_gpt_model.rs"]
mod chat_gpt_model;

#[path = "sercrets_manager_service.rs"]
mod sercrets_manager_service;

pub async fn ask_chat_gpt(question: &String) -> Result<String, Error> {
  let api_key = match sercrets_manager_service::get_api_key().await {
    Ok(value) => value,
    _ => "Not valid API key".to_string(),
  };

  return get_chat_gpt_reply(&question, &api_key).await;
}

async fn get_chat_gpt_reply(question: &String, api_key: &String) -> Result<String, Error> {
  const API_URL: &str = "https://api.openai.com/v1/chat/completions";

  const REQUEST_ROLE: &str = "user";
  const MODEL: &str = "gpt-3.5-turbo";

  // Prepare request data
  let authorization: String = format!("Bearer {api_key}");
  
  let content = &question;
  
  let request_body = chat_gpt_model::RequestBody {
    model: MODEL.to_string(),
    messages: [chat_gpt_model::RequestMessage {
      role: REQUEST_ROLE.to_string(),
      content: content.to_string(),
    }],
  };

  // Create client and send request
  let client = reqwest::Client::new();

  let response = client.post(API_URL)
    .json(&request_body)
    .header("Content-Type", "application/json")
    .header("Authorization", authorization)
    .send()
    .await?;

  // Get response body and parse
  let json_response = response
    .json::<chat_gpt_model::ResponseBody>()
    .await?;

  // Fetch the data we want
  let reply = String::from(&json_response.choices[0].message.content);

  return Ok(reply);
}
