use aes::Aes256;
use axum::{
    Router,
    body::Body,
    extract::{MatchedPath, Query},
    http::{StatusCode, header},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
};
use cbc::{
    Encryptor,
    cipher::{BlockEncryptMut, KeyIvInit, block_padding::Pkcs7},
};
use chrono::Local;
use common::logging::init_tracing;
use hmac::{Hmac, Mac};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    init_tracing(tracing::Level::DEBUG);
    let sdk_routes = Router::new()
        .route("/logstores/prod_lcx_sdk/shards/lb", post(logstores))
        .route(
            "/logstores/prod_lcx_request_data/shards/lb",
            post(logstores),
        )
        .route("/logstores/prod_score_data/shards/lb", post(logstores))
        .route("/environment", get(environment))
        .route("/module/init", get(mod_init))
        .route("/limbo/anti-fraud/init", get(afinit))
        .route("/api/v6.10/androidevent", post(androidevent))
        .route("/install_data/v5.0/com.dgames.g65002002.google", get(welp))
        .route("/public/realms", get(realms))
        .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    tracing::debug!("SDK listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, sdk_routes).await.unwrap();
}

async fn logstores(matched_path: MatchedPath) -> impl IntoResponse {
    tracing::debug!("Received POST request to {}", matched_path.as_str());
    StatusCode::OK
}

async fn androidevent(_matched_path: MatchedPath) -> impl IntoResponse {
    tracing::debug!("Received POST request");
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/plain")
        .body(Body::from("ok"))
        .unwrap()
}

type Aes256CbcEnc = Encryptor<Aes256>;

async fn realms() -> impl IntoResponse {
    let encrypted = "c643f0446bfcab86fd96859174736a5828c524409190a8c0a3ea3c6b9df1c1127801184acca78444995d9cdd4b150439ef00fee60ec167b6e57b1df9745f5427f0903ac63944b35cd9fd87a22de068181e03054a6fa39d8fd44d278d8cd1f05e5461a2a9b49526eddf4b3b9e739b3c68b1599373867b4ef3f26fee7b8b95f547101378f096cb0d614790beb482a6300a1f556d59b28670142404977d3c5b382036e9792aaed262f64ec62eb652907dca29400e17300afd74c16946c908bda74e7f867dd30352d30c3d4907f10d5de9912bb83abb24e2eb8fe207f7b1976f63e5846bf4eebf520841989a36a15795e9f5b64f5dcaff6b1dbc201f4aba7ff0d5b09d28cf866421061b23c755086bcd8caa";

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain")],
        encrypted.to_string(),
    )
}

async fn welp(
    Query(_params): Query<HashMap<String, String>>,
    matched_path: MatchedPath,
) -> impl IntoResponse {
    tracing::debug!("POST to {}", matched_path.as_str());
    let install_time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let resp = json!({
        "af_message": "organic install",
        "af_status": "Organic",
        "install_time": install_time
    });
    (StatusCode::OK, Json(resp))
}

async fn environment(matched_path: MatchedPath) -> impl IntoResponse {
    tracing::debug!("Received GET request to {}", matched_path.as_str());
    let resp = json!({
        "applicationId": "65002002",
        "isProd": true,
        "cs": {
            "url": "https://tickets-prod-ap-live.mobage.world/1",
            "accountCancellationUrl": "https://tickets-prod-ap-live.mobage.world/1"
        }
    });
    (StatusCode::OK, Json(resp))
}

async fn mod_init(
    Query(_params): Query<HashMap<String, String>>,
    matched_path: MatchedPath,
) -> impl IntoResponse {
    tracing::debug!("Received GET request to {}", matched_path.as_str());
    let resp = json!({
        "auth": [{"type": "DStore", "params": {}}],
        "ad": [
            {"type": "Facebook", "params": {"appid": "593332068848186"}},
            {"type": "AppsFlyer", "params": {"appleAppId": "", "appKey": "yQdMGTLck4jiDyS9aSybXQ"}}
        ],
        "share": [
            {"type": "Facebook", "params": {}},
            {"type": "Instagram", "params": {}},
            {"type": "Line", "params": {}},
            {"type": "System", "params": {}},
            {"type": "Twitter", "params": {}}
        ],
        "agreements": {
            "language": "en",
            "disable": false,
            "mainUrl": "https://lcmx-pp-cdn.mobage.world/65002002-prod/list.html",
            "title": null,
            "content": null,
            "version": "20230403153000",
            "isNative": false,
            "agreement": [
                {"title": "Privacy Policy", "url": "...", "type": "0"},
                {"title": "End User License Agreement", "url": "...", "type": "1"}
            ]
        },
        "cs": {
            "csUrl": "https://tickets-prod-ap-live.mobage.world/1",
            "accountCancellationUrl": "https://tickets-prod-ap-live.mobage.world/1"
        },
        "push": "FCM",
        "walletEnabled": true,
        "customizedUI": false,
        "disableRealNameVerify": false
    });
    (StatusCode::OK, Json(resp))
}

async fn afinit(
    Query(_params): Query<HashMap<String, String>>,
    matched_path: MatchedPath,
) -> impl IntoResponse {
    tracing::debug!("Received GET request to {}", matched_path.as_str());
    let resp = json!({"code": 10007, "message": "not support", "data": null});
    (StatusCode::OK, Json(resp))
}

async fn fallback(req: axum::http::Request<axum::body::Body>) -> impl IntoResponse {
    tracing::debug!("no route matched: {} {}", req.method(), req.uri());
    (StatusCode::NOT_FOUND, "no route")
}
