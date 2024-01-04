use reqwest::Client;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{from_value, json};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChatModel {
    GPT3,
    Gpt3Turbo,
    GPT4,
    Gpt4Turbo,
    Gpt3TurboInstruct,
    Babbage002,
    Davinci002,
}

impl ChatModel {
    fn name(&self) -> String {
        match self {
            ChatModel::GPT3 => "gpt-3",
            ChatModel::Gpt3Turbo => "gpt-3.5-turbo",
            ChatModel::GPT4 => "gpt-4",
            ChatModel::Gpt4Turbo => "gpt-4-turbo",
            ChatModel::Gpt3TurboInstruct => "gpt-3.5-turbo-instruct",
            ChatModel::Babbage002 => "babbage-002",
            ChatModel::Davinci002 => "davinci-002",
        }
        .to_string()
    }
}

fn serialize_openai_chat_model<S>(model: &ChatModel, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&model.name())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    #[serde(serialize_with = "serialize_openai_chat_model")]
    model: ChatModel,
    messages: Vec<Message>,
    frequency_penalty: Option<f32>,
    logit_bias: Option<f32>,
    logprobs: Option<i32>,
    top_logprobs: Option<i32>,
    max_tokens: Option<i32>,
    n: Option<i32>,
    presence_penalty: Option<f32>,
    response_format: Option<String>,
    seed: Option<i32>,
    stop: Option<Vec<String>>,
    stream: Option<bool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    tools: Option<Vec<String>>,
    tool_choice: Option<i32>,
    user: Option<String>,
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            model: ChatModel::Gpt3Turbo,
            messages: vec![],
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_tokens: None,
            n: None,
            presence_penalty: None,
            response_format: None,
            seed: None,
            stop: None,
            stream: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            user: None,
        }
    }
}

pub async fn chat(payload: Payload) -> Result<String, String> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = Client::new();

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!(payload))
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if response.status().is_success() {
        response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))
    } else {
        Err(format!(
            "API Request Failed: {}",
            response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string())
        ))
    }
}

pub async fn prompt(text: String, mut conversation: Vec<Message>) -> Result<Message, String> {
    conversation.push(Message {
        role: "user".to_string(),
        content: text,
    });

    let payload = Payload {
        messages: conversation,
        ..Default::default()
    };

    let response = chat(payload).await;
    match response {
        Ok(body) => {
            let body_value = serde_json::from_str::<serde_json::Value>(&body)
                .map_err(|_| "Error parsing response body")?;

            let first_choice = body_value["choices"][0].clone();
            let message_value = first_choice["message"].clone();
            from_value::<Message>(message_value).map_err(|_| "Error parsing response message".to_string())
        }
        Err(e) => Err(format!("Chat API error: {}", e)),
    }
}

pub fn prompt_sync(text: String, conversation: Vec<Message>) -> Result<Message, String> {
    let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create runtime: {}", e))?;

    rt.block_on(prompt(text, conversation))
}