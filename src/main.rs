mod web;


fn main() {
    let r = web::server::server();
    // Runs after the server comes down it seems
    match r {
        Ok(_) => println!("Server started!"),
        Err(e) => println!("Error: {}", e)
    }
}