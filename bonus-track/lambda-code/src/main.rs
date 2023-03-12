use tracing::log::{debug, info};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

#[path = "service/chat_gpt_service.rs"]
mod chat_gpt_service;
#[path = "helper/lambda_helper.rs"]
mod lambda_helper;
#[path = "model/lambda_model.rs"]
mod lambda_model;

async fn handler(request: LambdaEvent<lambda_model::RequestPayload>) -> lambda_model::Response {
  /*
  Please don't do this, it's a security issue, you don't know upfront what kind of data you will receive 
  in the payload and you might be exposing sensible data in logs
  */
  // debug!("request: {request:?}");

  // Get the prompt value from request payload
  let prompt: String = request.payload.prompt;

  info!("requested propmt: {prompt}");

  // Ask chat GPT
  let reply = match &chat_gpt_service::ask_chat_gpt(&prompt).await {
    Ok(content) => content.to_string(),
    _ => "Nothing happened".to_string(),
  };

  info!("received reply: {reply}");

  // Return received reply
  return Ok(lambda_model::SuccessResponse {
    body: reply,
  });
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  lambda_helper::setup_tracing();

  let func = service_fn(handler);

  run(func).await?;

  return Ok(());
}
