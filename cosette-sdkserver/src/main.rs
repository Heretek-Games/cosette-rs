use axum::{
    Router,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
};
use serde_json::{Value, json};
use std::collections::HashMap;
use tracing::Level;

pub fn init_tracing(level: Level) {
    eprintln!(
        r#"   ______                __  __
       / ____/___  ________  / /_/ /____        __________
      / /   / __ \/ ___/ _ \/ __/ __/ _ \______/ ___/ ___/
     / /___/ /_/ (__  )  __/ /_/ /_/  __/_____/ /  (__  )
     \____/\____/____/\___/\__/\__/\___/     /_/  /____/
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
        .route(
            "/api/v6.10/androidevent",
            post(logstores), /*.route(
                             "/install_data/v5.0/com.dgames.g65002002.google",
                             get(logstores),*/
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    tracing::debug!("SDK listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, sdk_routes).await.unwrap();
}

async fn logstores() -> impl IntoResponse {
    //idk man it had to be, else why would i store your data like cn devs :motorized_wheelchair:
    axum::http::StatusCode::OK
}

async fn environment() -> impl IntoResponse {
    let resp = json!({
        "applicationId": "65002002",
        "isProd": true,
        "cs": {
            "url": "https://tickets-prod-ap-live.mobage.world/1",
            "accountCancellationUrl": "https://tickets-prod-ap-live.mobage.world/1"
        }
    });

    (axum::http::StatusCode::OK, Json(resp))
}

async fn mod_init(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let resp = json!({
        "auth": [
            {
                "type": "DStore",
                "params": {}
            }
        ],
        "ad": [
            {
                "type": "Facebook",
                "params": {
                    "appid": "593332068848186"
                }
            },
            {
                "type": "AppsFlyer",
                "params": {
                    "appleAppId": "",
                    "appKey": "yQdMGTLck4jiDyS9aSybXQ"
                }
            }
        ],
        "share": [
            {
                "type": "Facebook",
                "params": {}
            },
            {
                "type": "Instagram",
                "params": {}
            },
            {
                "type": "Line",
                "params": {}
            },
            {
                "type": "System",
                "params": {}
            },
            {
                "type": "Twitter",
                "params": {}
            }
        ],
        "agreements": {
            "language": "en",
            "disable": false,
            "mainUrl": "https://lcmx-pp-cdn.mobage.world/65002002-prod/list.html?folder=ALL&regi
    on=ALL&filename=20230901112934%E3%80%90Score%E3%80%91%E3%80%90%E8%8B%B1%E8%AF%AD%E3%80%91%E3
    %80%90%E5%8C%97%E7%BE%8E%E7%AD%89%E5%9C%B0%E5%8C%BA%E3%80%91%E3%80%90%E9%9A%90%E7%A7%81%E5%8
    D%8F%E8%AE%AE%E3%80%91%2C20230901112217%E3%80%90Score%E3%80%91%E3%80%90%E8%8B%B1%E8%AF%AD%E3
    %80%91%E3%80%90%E6%AC%A7%E7%9B%9F%E5%8C%97%E7%BE%8E%E7%AD%89%E5%9C%B0%E5%8C%BA%E3%80%91%E3%8
    0%90%E7%94%A8%E6%88%B7%E5%8D%8F%E8%AE%AE%E3%80%91",
            "title": null,
            "content": null,
            "version": "20230403153000",
            "isNative": false,
            "agreement": [
                {
                    "title": "Privacy Policy",
                    "url": "https://lcmx-pp-cdn.mobage.world/65002002-prod/detail.html?folder=AL
    L&region=ALL&filename=20230901112934%E3%80%90Score%E3%80%91%E3%80%90%E8%8B%B1%E8%AF%AD%E3%80
    %91%E3%80%90%E5%8C%97%E7%BE%8E%E7%AD%89%E5%9C%B0%E5%8C%BA%E3%80%91%E3%80%90%E9%9A%90%E7%A7%8
    1%E5%8D%8F%E8%AE%AE%E3%80%91&filetype=0",
                    "type": "0"
                },
                {
                    "title": "End User License Agreement",
                    "url": "https://lcmx-pp-cdn.mobage.world/65002002-prod/detail.html?folder=AL
    L&region=ALL&filename=20230901112217%E3%80%90Score%E3%80%91%E3%80%90%E8%8B%B1%E8%AF%AD%E3%80
    %91%E3%80%90%E6%AC%A7%E7%9B%9F%E5%8C%97%E7%BE%8E%E7%AD%89%E5%9C%B0%E5%8C%BA%E3%80%91%E3%80%9
    0%E7%94%A8%E6%88%B7%E5%8D%8F%E8%AE%AE%E3%80%91&filetype=1",
                    "type": "1"
                }
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
    (axum::http::StatusCode::OK, Json(resp))
}

async fn afinit(Query(_params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let resp = json!({
        "code": 200,
        "message": "niggers",
        "data": null
    });
    (axum::http::StatusCode::OK, Json(resp))
}
