use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Mutex;

fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}

async fn index(data: web::Data<AppState>, num: web::Path<i32>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    let num = num.into_inner();

    HttpResponse::Ok().body(format!("Factorial of {}: {}", num, factorial(num)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/{num}", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}