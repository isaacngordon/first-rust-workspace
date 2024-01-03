use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

pub async fn chat(messages: Vec<Message>) -> Result<String, reqwest::Error> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = Client::new();

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": messages
        }))
        .send()
        .await?;

    response.text().await
}

pub async fn prompt(
    prompt: String,
    mut conversation: Conversation,
) -> Result<Message, &'static str> {
    conversation.messages.push(Message {
        role: "user".to_string(),
        content: prompt.to_string(),
    });

    let response = chat(conversation.messages).await;
    let body = serde_json::from_str::<serde_json::Value>(&response.unwrap()).unwrap();

    let first_choice = body["choices"][0].clone();
    let message_value = first_choice["message"].clone();
    let message = from_value::<Message>(message_value).map_err(|_| "Error parsing response")?;

    Ok(message)
}
