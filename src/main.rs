use core::llm::openai::Conversation;
use std::vec;

use crate::core::llm::openai;

mod core;
// fn main(){
//     println!("Hello, world!");
//     core::llm::openai::call_openai_api("key".to_string());
//     // core::my_shell::run();
// }

#[tokio::main]
async fn main() {
    let prompt = "Hello, is this thing on?".to_string();
    let conversation = Conversation {
        messages: vec![],
    };
    let response = openai::prompt(prompt, conversation).await;

    match response {
        Ok(msg) => println!("{}", msg.content),
        Err(error) => println!("Error: {}", error),
    }
}