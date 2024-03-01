use actix_web::{get, App, HttpServer, Responder};
use rand::seq::SliceRandom;

#[get("/")]
async fn index() -> impl Responder {
    let messages = vec![
        "Week 4 Mini Project: Actix-Web with Docker",
        "Hello, there!",
        "Welcome to the Oliver's website!",
        "Actix-Web and Docker make a great combination!",
        "Have a fantastic day!",
        "Explore the power of Rust programming language!",
        "Coding is an art, and you're the artist!",
        "The journey of a thousand miles begins with a single keystroke.",
        "In Rust we trust!",
        "May your code compile without errors!",
        "Web development with Actix-Web is a breeze!",
        "Keep calm and code on!",
    ];

    // Choose a random message from the list
    let random_message = messages.choose(&mut rand::thread_rng()).unwrap_or(&"No messages available");

    format!("Refresh to get your random message: {}", random_message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}