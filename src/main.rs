use actix_web::{get, post, HttpResponse, Responder, web::Json, HttpServer, App};
use std::time::Duration;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Info {
    pub message: String,
}

#[get("/health")]
pub async fn health_handler(req: Json<Info>) -> impl Responder {
    tokio::spawn( async {
        for _ in 0..5 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Health Good!...");
        }
    });
    HttpResponse::Ok().json(&json!({"hello": "world", "message": req.message}))
}

#[post("/action")]
pub async fn action_handler(req: Json<Info>) -> impl Responder {
    let handle = tokio::spawn(async {
        for _ in 0..5 {
            tokio::time::sleep(Duration::from_millis(50)).await;
            println!("Actions Good!...");
        }
    });
    if let Err(_) = handle.await {
        println!("Error 🤨");
        return HttpResponse::BadRequest().json(&json!({"error": "Error 🤨"}));
    }

    HttpResponse::Ok().json(&json!({"hello": "world", "message": req.message}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_handler)
            .service(action_handler)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
