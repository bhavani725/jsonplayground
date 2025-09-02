
use actix_web::{web, App, HttpServer,HttpRequest, Result, HttpResponse, middleware::Logger};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str, to_string_pretty};
use actix_web_prometheus::PrometheusMetrics;
use actix_web_prometheus::PrometheusMetricsBuilder;
use std::env;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use actix_files::NamedFile;

#[derive(Deserialize)]
struct JsonRequest {
    json_text: String,
    indent_size: Option<usize>,
}

#[derive(Serialize)]
struct JsonResponse {
    success: bool,
    formatted_json: Option<String>,
    error_message: Option<String>,
    is_valid: bool,
}

async fn validate_and_format(req: web::Json<JsonRequest>) -> Result<HttpResponse> {
    let json_text = req.json_text.trim();
    
    if json_text.is_empty() {
        return Ok(HttpResponse::Ok().json(JsonResponse {
            success: false,
            formatted_json: None,
            error_message: Some("Empty JSON input".to_string()),
            is_valid: false,
        }));
    }

    match from_str::<Value>(json_text) {
        Ok(parsed) => {
            let formatted = match to_string_pretty(&parsed) {
                Ok(pretty) => pretty,
                Err(e) => {
                    return Ok(HttpResponse::Ok().json(JsonResponse {
                        success: false,
                        formatted_json: None,
                        error_message: Some(format!("Formatting error: {}", e)),
                        is_valid: true,
                    }));
                }
            };

            Ok(HttpResponse::Ok().json(JsonResponse {
                success: true,
                formatted_json: Some(formatted),
                error_message: None,
                is_valid: true,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::Ok().json(JsonResponse {
                success: false,
                formatted_json: None,
                error_message: Some(format!("Invalid JSON: {}", e)),
                is_valid: false,
            }))
        }
    }
}

async fn minify_json(req: web::Json<JsonRequest>) -> Result<HttpResponse> {
    let json_text = req.json_text.trim();
    
    if json_text.is_empty() {
        return Ok(HttpResponse::Ok().json(JsonResponse {
            success: false,
            formatted_json: None,
            error_message: Some("Empty JSON input".to_string()),
            is_valid: false,
        }));
    }

    match from_str::<Value>(json_text) {
        Ok(parsed) => {
            let minified = match serde_json::to_string(&parsed) {
                Ok(compact) => compact,
                Err(e) => {
                    return Ok(HttpResponse::Ok().json(JsonResponse {
                        success: false,
                        formatted_json: None,
                        error_message: Some(format!("Minification error: {}", e)),
                        is_valid: true,
                    }));
                }
            };

            Ok(HttpResponse::Ok().json(JsonResponse {
                success: true,
                formatted_json: Some(minified),
                error_message: None,
                is_valid: true,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::Ok().json(JsonResponse {
                success: false,
                formatted_json: None,
                error_message: Some(format!("Invalid JSON: {}", e)),
                is_valid: false,
            }))
        }
    }
}


async fn serve_static_file(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}
async fn serve_index() -> Result<HttpResponse> {
    let html_content = r#"

    "#;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content))
}

//#[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init();
    
//     println!("🚀 Starting JSON Validator & Formatter Server...");
//     println!("📍 Server will be available at: http://localhost:8080");
    
//     HttpServer::new(|| {
//         App::new()
//             .wrap(Logger::default())
//             .route("/", web::get().to(serve_index))
//             .route("/api/format", web::post().to(validate_and_format))
//             .route("/api/minify", web::post().to(minify_json))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();


        // let metrics = web::Data::new(AppMetrics::new());
        
let prometheus = PrometheusMetricsBuilder::new("json_validator")
    .endpoint("/metrics")
    .build()
    .unwrap();
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // Get configuration from environment
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let workers = env::var("WORKERS")
        .unwrap_or_else(|_| num_cpus::get().to_string())
        .parse::<usize>()
            .unwrap_or(num_cpus::get());
    
    log::info!("🚀 Starting JSON Validator Server");
    log::info!("📍 Server binding to: {}:{}", host, port);
    log::info!("👥 Using {} worker threads", workers);
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            //.route("/", web::get().to(serve_index))
            .route("/", web::get().to(serve_static_file))
            .route("/health", web::get().to(health_check))
            .route("/metrics", web::get().to(metrics))
            .route("/api/format", web::post().to(validate_and_format))
            .route("/api/minify", web::post().to(minify_json))
    })
    .workers(workers)
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

#[derive(Clone)]
pub struct AppMetrics {
    pub requests_total: Arc<AtomicU64>,
    pub format_requests: Arc<AtomicU64>,
    pub minify_requests: Arc<AtomicU64>,
    pub errors_total: Arc<AtomicU64>,
}

impl AppMetrics {
    pub fn new() -> Self {
        Self {
            requests_total: Arc::new(AtomicU64::new(0)),
            format_requests: Arc::new(AtomicU64::new(0)),
            minify_requests: Arc::new(AtomicU64::new(0)),
            errors_total: Arc::new(AtomicU64::new(0)),
        }
    }
 }
 
 async fn metrics(data: web::Data<AppMetrics>) -> Result<HttpResponse> {
    let metrics = format!(
        "# HELP requests_total Total number of requests\n\
         # TYPE requests_total counter\n\
         requests_total {}\n\
         # HELP format_requests_total Total number of format requests\n\
         # TYPE format_requests_total counter\n\
         format_requests_total {}\n\
         # HELP minify_requests_total Total number of minify requests\n\
         # TYPE minify_requests_total counter\n\
         minify_requests_total {}\n\
         # HELP errors_total Total number of errors\n\
         # TYPE errors_total counter\n\
         errors_total {}\n",
        data.requests_total.load(Ordering::Relaxed),
        data.format_requests.load(Ordering::Relaxed),
        data.minify_requests.load(Ordering::Relaxed),
        data.errors_total.load(Ordering::Relaxed),
    );

    Ok(HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4; charset=utf-8")
        .body(metrics))
}

async fn health_check() -> Result<HttpResponse> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": timestamp,
        "service": "json-validator"
    })))
}