use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct PingResponse {
    message: String,
    timestamp: String,
    lang: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AdditionRequest {
    a: i32,
    b: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AdditionResponse {
    result: i32,
    a: i32,
    b: i32,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

async fn wagmi_handler(
    req: web::Json<Value>,
) -> impl Responder {
    // Check if the request is empty or has no fields
    if req.is_object() && req.as_object().unwrap().is_empty() {
        let response = PingResponse {
            message: "wagmi".to_string(),
            timestamp: Utc::now().to_rfc3339(),
            lang: "Rust".to_string(),
        };
        return HttpResponse::Ok().json(response);
    }

    // Try to parse as AdditionRequest
    match serde_json::from_value::<AdditionRequest>(req.into_inner()) {
        Ok(req) => {
            if req.a < 0 || req.b < 0 || req.a + req.b > 100 {
                let error = ErrorResponse {
                    error: "Invalid input".to_string(),
                };
                return HttpResponse::BadRequest().json(error);
            }

            let response = AdditionResponse {
                result: req.a + req.b,
                a: req.a,
                b: req.b,
                status: "success".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let error = ErrorResponse {
                error: "Invalid input".to_string(),
            };
            HttpResponse::BadRequest().json(error)
        }
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    println!("Starting WAGMI-9000 server...");
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().expect("PORT must be a number");
    
    println!("Server will run on port: {}", port);
    
    HttpServer::new(|| {
        App::new()
            .route("/wagmi", web::post().to(wagmi_handler))
            .route("/health", web::get().to(health_check))
    })
    .workers(num_cpus::get() * 2) // Optimize for concurrent requests
    .keep_alive(std::time::Duration::from_secs(30)) // Keep connections alive
    .client_request_timeout(std::time::Duration::from_secs(5)) // Set request timeout
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
