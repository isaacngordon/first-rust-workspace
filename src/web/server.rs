use openai::chat;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use std::sync::Mutex;

async fn p() -> String {
    let res = chat::prompt(
            String::from("MARCO!"),
            vec![]   
    ).await;

    match res {
        Ok(msg) => return format!("Response: {}", msg.content),
        Err(e) => return format!("Error: {}", e)
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(p().await)
}

struct Counter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: actix_web::web::Data<Counter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {

     // Note: web::Data created _outside_ HttpServer::new closure
     let counter = web::Data::new(Counter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
        .app_data(counter.clone()) // <- register the created data
        // .service(hello)
        .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await
}