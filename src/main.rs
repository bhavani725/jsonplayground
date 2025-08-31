
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
            max-width: 1400px;
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
            grid-template-columns: 1fr 1fr 1fr;
            gap: 20px;
            margin-bottom: 30px;
        }
        
        .input-section, .output-section, .tree-section {
            background: #f8f9fa;
            border-radius: 15px;
            padding: 25px;
            border: 2px solid #e9ecef;
            transition: all 0.3s ease;
        }
        
        .input-section:hover, .output-section:hover, .tree-section:hover {
            border-color: #667eea;
            box-shadow: 0 5px 15px rgba(102, 126, 234, 0.1);
        }
        
        .section-title {
            font-size: 1.3em;
            font-weight: 600;
            margin-bottom: 15px;
            color: #2c3e50;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        
        .view-toggle {
            display: flex;
            gap: 5px;
        }
        
        .toggle-btn {
            padding: 4px 8px;
            border: 1px solid #ccc;
            background: white;
            border-radius: 4px;
            font-size: 0.8em;
            cursor: pointer;
            transition: all 0.3s ease;
        }
        
        .toggle-btn.active {
            background: #667eea;
            color: white;
            border-color: #667eea;
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
        
        .tree-view {
            height: 350px;
            border: 2px solid #dee2e6;
            border-radius: 10px;
            background: white;
            overflow: auto;
            padding: 15px;
            font-family: 'Monaco', 'Courier New', monospace;
            font-size: 13px;
            line-height: 1.4;
        }
        
        .tree-node {
            margin: 2px 0;
            position: relative;
        }
        
        .tree-key {
            color: #0451a5;
            font-weight: 600;
        }
        
        .tree-string {
            color: #0a4a72;
        }
        
        .tree-number {
            color: #d73a49;
            font-weight: 500;
        }
        
        .tree-boolean {
            color: #005cc5;
            font-weight: 600;
        }
        
        .tree-null {
            color: #6f42c1;
            font-weight: 600;
        }
        
        .tree-bracket {
            color: #24292e;
            font-weight: bold;
        }
        
        .tree-toggle {
            cursor: pointer;
            user-select: none;
            display: inline-block;
            width: 16px;
            text-align: center;
            margin-right: 4px;
            font-weight: bold;
            color: #586069;
        }
        
        .tree-toggle:hover {
            background: #e1e4e8;
            border-radius: 3px;
        }
        
        .tree-toggle.expanded::before {
            content: '▼';
        }
        
        .tree-toggle.collapsed::before {
            content: '▶';
        }
        
        .tree-toggle.leaf::before {
            content: '•';
            color: #d1d5da;
        }
        
        .tree-children {
            margin-left: 20px;
            border-left: 1px solid #e1e4e8;
            padding-left: 10px;
        }
        
        .tree-children.collapsed {
            display: none;
        }
        
        .tree-path {
            font-size: 0.8em;
            color: #586069;
            background: #f6f8fa;
            padding: 2px 6px;
            border-radius: 3px;
            margin-left: 8px;
        }
        
        .tree-count {
            font-size: 0.8em;
            color: #586069;
            font-style: italic;
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
            
            .view-toggle {
                margin-top: 10px;
            }
            
            .tree-section {
                display: none !important;
            }
        }
        
        @media (max-width: 1200px) and (min-width: 769px) {
            .editor-section {
                grid-template-columns: 1fr 1fr;
            }
            
            .tree-section {
                grid-column: span 2;
                display: block !important;
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
                    <h3 class="section-title">
                        ✨ Formatted Output
                        <div class="view-toggle">
                            <button class="toggle-btn active" onclick="switchView('text')">Text</button>
                            <button class="toggle-btn" onclick="switchView('tree')">Tree</button>
                        </div>
                    </h3>
                    <textarea id="outputJson" placeholder="Formatted JSON will appear here..." readonly></textarea>
                    <div id="treeView" class="tree-view" style="display: none;"></div>
                </div>
                
                <div class="tree-section" id="treeSection" style="display: none;">
                    <h3 class="section-title">🌳 Interactive Tree View</h3>
                    <div id="interactiveTree" class="tree-view"></div>
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
        let currentJsonData = null;
        
        function showStatus(message, isError = false) {
            const status = document.getElementById('status');
            status.textContent = message;
            status.className = `status ${isError ? 'status-error' : 'status-success'} show`;
            
            setTimeout(() => {
                status.classList.remove('show');
            }, 4000);
        }
        
        function switchView(viewType) {
            const textView = document.getElementById('outputJson');
            const treeView = document.getElementById('treeView');
            const buttons = document.querySelectorAll('.toggle-btn');
            
            buttons.forEach(btn => btn.classList.remove('active'));
            
            if (viewType === 'text') {
                textView.style.display = 'block';
                treeView.style.display = 'none';
                buttons[0].classList.add('active');
            } else {
                textView.style.display = 'none';
                treeView.style.display = 'block';
                buttons[1].classList.add('active');
                if (currentJsonData) {
                    renderTreeView(currentJsonData, treeView);
                }
            }
        }
        
        function createTreeNode(key, value, path = '', isLast = false, isRoot = false) {
            const nodeDiv = document.createElement('div');
            nodeDiv.className = 'tree-node';
            
            if (value === null) {
                nodeDiv.innerHTML = `
                    <span class="tree-toggle leaf"></span>
                    ${key ? `<span class="tree-key">"${key}"</span>: ` : ''}
                    <span class="tree-null">null</span>
                `;
            } else if (typeof value === 'string') {
                nodeDiv.innerHTML = `
                    <span class="tree-toggle leaf"></span>
                    ${key ? `<span class="tree-key">"${key}"</span>: ` : ''}
                    <span class="tree-string">"${escapeHtml(value)}"</span>
                `;
            } else if (typeof value === 'number') {
                nodeDiv.innerHTML = `
                    <span class="tree-toggle leaf"></span>
                    ${key ? `<span class="tree-key">"${key}"</span>: ` : ''}
                    <span class="tree-number">${value}</span>
                `;
            } else if (typeof value === 'boolean') {
                nodeDiv.innerHTML = `
                    <span class="tree-toggle leaf"></span>
                    ${key ? `<span class="tree-key">"${key}"</span>: ` : ''}
                    <span class="tree-boolean">${value}</span>
                `;
            } else if (Array.isArray(value)) {
                const toggle = document.createElement('span');
                toggle.className = 'tree-toggle expanded';
                toggle.onclick = () => toggleNode(toggle);
                
                const content = document.createElement('span');
                content.innerHTML = `
                    ${key ? `<span class="tree-key">"${key}"</span>: ` : ''}
                    <span class="tree-bracket">[</span>
                    <span class="tree-count">${value.length} items</span>
                `;
                
                nodeDiv.appendChild(toggle);
                nodeDiv.appendChild(content);
                
                if (value.length > 0) {
                    const childrenDiv = document.createElement('div');
                    childrenDiv.className = 'tree-children';
                    
                    value.forEach((item, index) => {
                        const isLastItem = index === value.length - 1;
                        const childPath = path ? `${path}[${index}]` : `[${index}]`;
                        const childNode = createTreeNode(`${index}`, item, childPath, isLastItem);
                        childrenDiv.appendChild(childNode);
                    });
                    
                    nodeDiv.appendChild(childrenDiv);
                }
                
                const closingBracket = document.createElement('div');
                closingBracket.className = 'tree-node';
                closingBracket.innerHTML = '<span class="tree-toggle leaf"></span><span class="tree-bracket">]</span>';
                nodeDiv.appendChild(closingBracket);
                
            } else if (typeof value === 'object') {
                const keys = Object.keys(value);
                const toggle = document.createElement('span');
                toggle.className = 'tree-toggle expanded';
                toggle.onclick = () => toggleNode(toggle);
                
                const content = document.createElement('span');
                content.innerHTML = `
                    ${key ? `<span class="tree-key">"${key}"</span>: ` : ''}
                    <span class="tree-bracket">{</span>
                    <span class="tree-count">${keys.length} properties</span>
                `;
                
                nodeDiv.appendChild(toggle);
                nodeDiv.appendChild(content);
                
                if (keys.length > 0) {
                    const childrenDiv = document.createElement('div');
                    childrenDiv.className = 'tree-children';
                    
                    keys.forEach((objKey, index) => {
                        const isLastItem = index === keys.length - 1;
                        const childPath = path ? `${path}.${objKey}` : objKey;
                        const childNode = createTreeNode(objKey, value[objKey], childPath, isLastItem);
                        childrenDiv.appendChild(childNode);
                    });
                    
                    nodeDiv.appendChild(childrenDiv);
                }
                
                const closingBrace = document.createElement('div');
                closingBrace.className = 'tree-node';
                closingBrace.innerHTML = '<span class="tree-toggle leaf"></span><span class="tree-bracket">}</span>';
                nodeDiv.appendChild(closingBrace);
            }
            
            return nodeDiv;
        }
        
        function toggleNode(toggleElement) {
            const isExpanded = toggleElement.classList.contains('expanded');
            const node = toggleElement.parentElement;
            const children = node.querySelector('.tree-children');
            
            if (children) {
                if (isExpanded) {
                    toggleElement.classList.remove('expanded');
                    toggleElement.classList.add('collapsed');
                    children.classList.add('collapsed');
                } else {
                    toggleElement.classList.remove('collapsed');
                    toggleElement.classList.add('expanded');
                    children.classList.remove('collapsed');
                }
            }
        }
        
        function renderTreeView(jsonData, container) {
            container.innerHTML = '';
            const treeNode = createTreeNode(null, jsonData, '', false, true);
            container.appendChild(treeNode);
        }
        
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
        
        function updateTreeSection() {
            const treeSection = document.getElementById('treeSection');
            const interactiveTree = document.getElementById('interactiveTree');
            
            if (currentJsonData) {
                if (window.innerWidth > 768) {
                    treeSection.style.display = 'block';
                    renderTreeView(currentJsonData, interactiveTree);
                } else {
                    treeSection.style.display = 'none';
                }
            }
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
                    currentJsonData = JSON.parse(inputJson);
                    updateTreeSection();
                    showStatus('✅ JSON is valid and has been formatted successfully!');
                } else {
                    outputJson.value = '';
                    currentJsonData = null;
                    document.getElementById('treeView').innerHTML = '';
                    document.getElementById('interactiveTree').innerHTML = '';
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
                    currentJsonData = JSON.parse(inputJson);
                    updateTreeSection();
                    showStatus('📦 JSON has been minified successfully!');
                } else {
                    outputJson.value = '';
                    currentJsonData = null;
                    document.getElementById('treeView').innerHTML = '';
                    document.getElementById('interactiveTree').innerHTML = '';
                    showStatus(`❌ ${result.error_message}`, true);
                }
            } catch (error) {
                showStatus(`Network error: ${error.message}`, true);
            }
        }
        
        function clearAll() {
            document.getElementById('inputJson').value = '';
            document.getElementById('outputJson').value = '';
            document.getElementById('treeView').innerHTML = '';
            document.getElementById('interactiveTree').innerHTML = '';
            document.getElementById('treeSection').style.display = 'none';
            currentJsonData = null;
            showStatus('🗑️ Cleared all content!');
        }
        
        // Add some sample JSON for demo
        document.addEventListener('DOMContentLoaded', function() {
            const sampleJson = ``;
            document.getElementById('inputJson').value = sampleJson;
            
            // Handle window resize
            window.addEventListener('resize', updateTreeSection);
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
    env_logger::init();
    
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