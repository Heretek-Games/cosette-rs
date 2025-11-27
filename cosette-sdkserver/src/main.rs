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
use hex;
use hmac::{Hmac, Mac};
use rand::RngCore;
use serde_json::json;
use sha2::Sha256;
use snap::raw::Encoder;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tracing::Level;

pub fn init_tracing(level: Level) {
    eprintln!(
        r#" ______ __ __
       / ____/___ ________ / /_/ /____ __________
      / / / __ \/ ___/ _ \/ __/ __/ _ \______/ ___/ ___/
     / /___/ /_/ (__ ) __/ /_/ /_/ __/_____/ / (__ )
     \____/\____/____/\___/\__/\__/\___/ /_/ /____/
                                                          "#
    );
    tracing_subscriber::fmt().with_max_level(level).init()
}

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

async fn realms(
    Query(params): Query<HashMap<String, String>>,
    _: MatchedPath,
) -> impl IntoResponse {
    let json_bytes = json!({
        "masterDataVersion": "2025.11.25-rc1",
        "masterDataUrl": "http://127.0.0.1:5000/masterdata.json",
        "needUpdateMasterData": true,
        "needUpdateAssetBundle": true,
        "hasDirtyFile": false,
        "assetsBundleVersion": "v1.2.3",
        "assetsBundleUrl": "http://127.0.0.1:5000/bundles.zip",
        "pwd": null,
        "domain": "127.0.0.1:5000",
        "backupMasterDataUrl": "http://127.0.0.1:5000/masterdata.json",
        "backupAssetsBundleUrl": "http://127.0.0.1:5000/bundles.zip",
        "backupDomain": "127.0.0.1:5000"
    })
    .to_string()
    .into_bytes();

    // Sort query params alphabetically — REQUIRED FOR HMAC
    let mut sorted: Vec<_> = params.into_iter().collect();
    sorted.sort_by_key(|x| x.0.clone());
    let query_string = sorted
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);

    let key =
        hex::decode("39795577676558545840757942425e6a2572526253564456556a456e48556a68").unwrap();

    // HMAC
    let mut mac = Hmac::<Sha256>::new_from_slice(&key).unwrap();
    mac.update(&iv);
    mac.update(query_string.as_bytes());
    let hmac = mac.finalize().into_bytes();

    // Raw Snappy
    let compressed = Encoder::new().compress_vec(&json_bytes).unwrap();

    println!(
        "JSON {} → raw Snappy {} bytes",
        json_bytes.len(),
        compressed.len()
    );
    println!(
        "First 10 bytes: {}",
        hex::encode(&compressed[..10.min(compressed.len())])
    );

    // Payload = HMAC + raw Snappy
    let mut payload = Vec::with_capacity(32 + compressed.len());
    payload.extend_from_slice(&hmac);
    payload.extend_from_slice(&compressed);

    // FINAL WORKING ENCRYPTION — WORKS WITH cbc 0.1.2
    let cipher = Aes256CbcEnc::new(&key.into(), &iv.into());
    let mut encrypted = payload.clone(); // clone to mutable buffer
    let encrypted_len = cipher
        .encrypt_padded::<Pkcs7>(&mut encrypted, payload.len())
        .unwrap()
        .len();

    let mut final_packet = Vec::with_capacity(16 + encrypted_len);
    final_packet.extend_from_slice(&iv);
    final_packet.extend_from_slice(&encrypted[..encrypted_len]);

    let response = hex::encode(final_packet);

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain")],
        response,
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
