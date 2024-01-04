use openai::chat;
use tokio::runtime::Runtime;

fn main() {
    let res = Runtime::new().unwrap().block_on(
        chat::prompt(
            String::from("MARCO!"),
            vec![]   
        )
    );

    match res {
        Ok(msg) => println!("Response: {}", msg.content),
        Err(e) => println!("Error: {}", e),
    }
}