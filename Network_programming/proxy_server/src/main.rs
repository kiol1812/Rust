use std::net::SocketAddr;
use tower::make::Shared;
use modsecurity::{ModSecurity, Rules};
use hyper::StatusCode;


use hyper::{service::service_fn, Body, Client, Request, Response, Server};

async fn log(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();

    if path.starts_with("/api") {
        println!("API Path: {}", path);
    } else {
        println!("Generic Path: {}", path);
    }

    // 處理判斷請求是否惡意
    // 建立 ModSecurity 實例
    let ms = ModSecurity::default();

    // 創建並添加規則
    let mut rules = Rules::new();
    rules
        .add_plain(r#"
            SecRuleEngine On

            SecRule REQUEST_URI "@rx admin" "id:1,phase:1,deny,status:401"
        "#)
        .expect("添加規則失敗");
    handle_client(req, &ms, &rules).await
}

/// 處理單一連線
async fn handle_client(req: Request<Body>, ms: &ModSecurity, rules: &Rules) -> Result<Response<Body>, hyper::Error> {
    // let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let (parts, body) = req.into_parts();
    println!("收到請求：\nheaders: {:?}\nextensions: {:?}\nmethod: {}\nversion: {:?}", parts.headers, parts.extensions, parts.method, parts.version);
    let body_bytes = hyper::body::to_bytes(body).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    println!("body: \n{}", body_str);

    // 解析第一行請求 (例如："GET /admin HTTP/1.1")
    let method_clone = parts.method.clone();
    let method = method_clone.as_str();
    let uri_clone = parts.uri.clone();
    let uri = uri_clone.path();
    let version_clone = parts.version.clone();
    let http_version = format!("{:?}", version_clone);

    // 建立 modsecurity 交易 (transaction)
    let mut transaction = ms
        .transaction_builder()
        .with_rules(rules)
        .build()
        .expect("建立交易失敗");

    // 處理 URI
    transaction
        .process_uri(uri, method, &http_version)
        .expect("處理 URI 失敗");

    // 處理請求標頭 (這裡僅呼叫此方法以模擬完整流程)
    transaction
        .process_request_headers()
        .expect("處理請求標頭失敗");

    // 檢查是否有攔截（intervention）
    let (status_code, body) = if let Some(intervention) = transaction.intervention() {
        if intervention.status() == 401 {
            (401, "Unauthorized")
        } else {
            (intervention.status(), "to-do🤪")
        }
    } else {
        (200, "pass")
    };

    if status_code!=200 { // block request with 403 forbinden
        let res = hyper::Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from(format!("{}\n", body)))
            .unwrap();
        return Ok(res);
    }
    
    // 判斷非惡意請求，重組請求並發送給server
    let client = Client::new();
    let req: Request<Body> = Request::from_parts(parts, Body::from(body_bytes)); // re-construct original request
    client.request(req).await // pass to server
}

#[tokio::main]
async fn main() {
    let make_service = Shared::new(service_fn(log));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}