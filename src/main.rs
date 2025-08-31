
use actix_web::{web, App, HttpServer, Result, HttpResponse, middleware::Logger};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_str, to_string_pretty};

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

async fn serve_index() -> Result<HttpResponse> {
    let html_content = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>JSON Validator & Formatter</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }
        
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }
        
        .header {
            background: linear-gradient(135deg, #ff6b6b, #ee5a24);
            color: white;
            padding: 30px;
            text-align: center;
        }
        
        .header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            font-weight: 700;
        }
        
        .header p {
            font-size: 1.1em;
            opacity: 0.9;
        }
        
        .main-content {
            padding: 40px;
        }
        
        .editor-section {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 30px;
            margin-bottom: 30px;
        }
        
        .input-section, .output-section {
            background: #f8f9fa;
            border-radius: 15px;
            padding: 25px;
            border: 2px solid #e9ecef;
            transition: all 0.3s ease;
        }
        
        .input-section:hover, .output-section:hover {
            border-color: #667eea;
            box-shadow: 0 5px 15px rgba(102, 126, 234, 0.1);
        }
        
        .section-title {
            font-size: 1.3em;
            font-weight: 600;
            margin-bottom: 15px;
            color: #2c3e50;
        }
        
        textarea {
            width: 100%;
            height: 350px;
            border: 2px solid #dee2e6;
            border-radius: 10px;
            padding: 15px;
            font-family: 'Monaco', 'Courier New', monospace;
            font-size: 14px;
            line-height: 1.5;
            resize: vertical;
            transition: all 0.3s ease;
            background: white;
        }
        
        textarea:focus {
            outline: none;
            border-color: #667eea;
            box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }
        
        .controls {
            display: flex;
            justify-content: center;
            gap: 15px;
            margin-bottom: 30px;
            flex-wrap: wrap;
        }
        
        .btn {
            padding: 12px 24px;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            text-transform: uppercase;
            letter-spacing: 1px;
        }
        
        .btn-primary {
            background: linear-gradient(135deg, #667eea, #764ba2);
            color: white;
        }
        
        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(102, 126, 234, 0.3);
        }
        
        .btn-secondary {
            background: linear-gradient(135deg, #ffeaa7, #fdcb6e);
            color: #2d3436;
        }
        
        .btn-secondary:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(253, 203, 110, 0.3);
        }
        
        .btn-danger {
            background: linear-gradient(135deg, #fd79a8, #e84393);
            color: white;
        }
        
        .btn-danger:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(232, 67, 147, 0.3);
        }
        
        .status {
            margin: 20px 0;
            padding: 15px 20px;
            border-radius: 10px;
            text-align: center;
            font-weight: 600;
            opacity: 0;
            transform: translateY(-10px);
            transition: all 0.3s ease;
        }
        
        .status.show {
            opacity: 1;
            transform: translateY(0);
        }
        
        .status-success {
            background: linear-gradient(135deg, #00b894, #00a085);
            color: white;
        }
        
        .status-error {
            background: linear-gradient(135deg, #e17055, #d63031);
            color: white;
        }
        
        .footer {
            background: #2d3436;
            color: white;
            text-align: center;
            padding: 20px;
            font-size: 0.9em;
        }
        
        @media (max-width: 768px) {
            .editor-section {
                grid-template-columns: 1fr;
            }
            
            .header h1 {
                font-size: 2em;
            }
            
            .main-content {
                padding: 20px;
            }
            
            .controls {
                flex-direction: column;
                align-items: center;
            }
            
            .btn {
                width: 200px;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔧 JSON Validator & Formatter</h1>
            <p>Validate, format, and beautify your JSON with ease</p>
        </div>
        
        <div class="main-content">
            <div class="editor-section">
                <div class="input-section">
                    <h3 class="section-title">📝 Input JSON</h3>
                    <textarea id="inputJson" placeholder="Paste your JSON here..."></textarea>
                </div>
                
                <div class="output-section">
                    <h3 class="section-title">✨ Formatted Output</h3>
                    <textarea id="outputJson" placeholder="Formatted JSON will appear here..." readonly></textarea>
                </div>
            </div>
            
            <div class="controls">
                <button class="btn btn-primary" onclick="formatJson()">
                    🎨 Format & Validate
                </button>
                <button class="btn btn-secondary" onclick="minifyJson()">
                    📦 Minify
                </button>
                <button class="btn btn-danger" onclick="clearAll()">
                    🗑️ Clear All
                </button>
            </div>
            
            <div id="status" class="status"></div>
        </div>
        
        <div class="footer">
            <p>Made with ❤️ using Rust & Actix-web | JSON processing powered by serde_json</p>
        </div>
    </div>

    <script>
        function showStatus(message, isError = false) {
            const status = document.getElementById('status');
            status.textContent = message;
            status.className = `status ${isError ? 'status-error' : 'status-success'} show`;
            
            setTimeout(() => {
                status.classList.remove('show');
            }, 4000);
        }
        
        async function formatJson() {
            const inputJson = document.getElementById('inputJson').value;
            const outputJson = document.getElementById('outputJson');
            
            if (!inputJson.trim()) {
                showStatus('Please enter some JSON to format!', true);
                return;
            }
            
            try {
                const response = await fetch('/api/format', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        json_text: inputJson
                    })
                });
                
                const result = await response.json();
                
                if (result.success) {
                    outputJson.value = result.formatted_json;
                    showStatus('✅ JSON is valid and has been formatted successfully!');
                } else {
                    outputJson.value = '';
                    showStatus(`❌ ${result.error_message}`, true);
                }
            } catch (error) {
                showStatus(`Network error: ${error.message}`, true);
            }
        }
        
        async function minifyJson() {
            const inputJson = document.getElementById('inputJson').value;
            const outputJson = document.getElementById('outputJson');
            
            if (!inputJson.trim()) {
                showStatus('Please enter some JSON to minify!', true);
                return;
            }
            
            try {
                const response = await fetch('/api/minify', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        json_text: inputJson
                    })
                });
                
                const result = await response.json();
                
                if (result.success) {
                    outputJson.value = result.formatted_json;
                    showStatus('📦 JSON has been minified successfully!');
                } else {
                    outputJson.value = '';
                    showStatus(`❌ ${result.error_message}`, true);
                }
            } catch (error) {
                showStatus(`Network error: ${error.message}`, true);
            }
        }
        
        function clearAll() {
            document.getElementById('inputJson').value = '';
            document.getElementById('outputJson').value = '';
            showStatus('🗑️ Cleared all content!');
        }
        
        // Add some sample JSON for demo
        document.addEventListener('DOMContentLoaded', function() {
            const sampleJson = `{
"name": "John Doe",
"age": 30,
"city": "New York",
"hobbies": ["reading", "swimming", "coding"],
"address": {
"street": "123 Main St",
"zipCode": "10001"
},
"isActive": true
}`;
            document.getElementById('inputJson').value = sampleJson;
        });
    </script>
</body>
</html>
    "#;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   //    env_logger::init();
    
    println!("🚀 Starting JSON Validator & Formatter Server...");
    println!("📍 Server will be available at: http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(serve_index))
            .route("/api/format", web::post().to(validate_and_format))
            .route("/api/minify", web::post().to(minify_json))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}