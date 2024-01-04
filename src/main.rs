use core::llm::openai;
use tokio::runtime::Runtime;

fn main() {
    let res = Runtime::new().unwrap().block_on(
        openai::prompt(
            String::from("MARCO!"),
            openai::Conversation { messages: vec![] }    
        )
    );

    match res {
        Ok(msg) => println!("Response: {}", msg.content),
        Err(e) => println!("Error: {}", e),
    }
}