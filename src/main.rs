use core::llm::openai::Conversation;
use std::vec;

use crate::core::llm::openai;

mod core;
// fn main(){
//     println!("Hello, world!");
//     core::llm::openai::call_openai_api("key".to_string());
//     // core::my_shell::run();
// }

fn print_message(msg: &openai::Message) {
    let emoji = match msg.role.as_str() {
        "assistant" => "🤖",
        "user" => "👤",
        "system" => "🖥️",
        _ => "👽",
    };
    println!("{} {} says: {}", emoji, msg.role, msg.content)
}

#[tokio::main]
async fn main() {
    let prompt = "Hello, is this thing on?".to_string();
    let conversation = Conversation {
        messages: vec![],
    };

    print_message(&openai::Message {
        role: "user".to_string(),
        content: prompt.clone(),
    });
    
    let response = openai::prompt(prompt, conversation).await;

    match response {
        Ok(msg) => print_message(&msg),
        Err(error) => println!("Error: {}", error),
    }
}