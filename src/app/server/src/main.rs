mod server;

fn main() {
    let r = server::start_server();
    // Runs after the server comes down it seems
    match r {
        Ok(_) => println!("Server started!"),
        Err(e) => println!("Error: {}", e)
    }
}