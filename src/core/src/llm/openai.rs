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
pub struct Conversation {
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
enum OpenAIChatModel {
    GPT3,
    Gpt3Turbo,
    GPT4,
    Gpt4Turbo,
    Gpt3TurboInstruct,
    Babbage002,
    Davinci002,
}

impl OpenAIChatModel {
    fn name(&self) -> String {
        match self {
            OpenAIChatModel::GPT3 => "gpt-3",
            OpenAIChatModel::Gpt3Turbo => "gpt-3.5-turbo",
            OpenAIChatModel::GPT4 => "gpt-4",
            OpenAIChatModel::Gpt4Turbo => "gpt-4-turbo",
            OpenAIChatModel::Gpt3TurboInstruct => "gpt-3.5-turbo-instruct",
            OpenAIChatModel::Babbage002 => "babbage-002",
            OpenAIChatModel::Davinci002 => "davinci-002",
        }
        .to_string()
    }
}

fn serialize_openai_chat_model<S>(model: &OpenAIChatModel, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&model.name())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIChatConfig {
    #[serde(serialize_with = "serialize_openai_chat_model")]
    model: OpenAIChatModel,
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

impl Default for OpenAIChatConfig {
    fn default() -> Self {
        OpenAIChatConfig {
            model: OpenAIChatModel::Gpt3Turbo,
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

pub async fn chat(messages: Vec<Message>) -> Result<String, String> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = Client::new();

    let config = OpenAIChatConfig {
        messages,
        ..Default::default()
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!(config))
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

pub async fn prompt(prompt: String, mut conversation: Conversation) -> Result<Message, String> {
    conversation.messages.push(Message {
        role: "user".to_string(),
        content: prompt,
    });

    let response = chat(conversation.messages).await;
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
