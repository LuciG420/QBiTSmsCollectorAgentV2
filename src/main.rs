use redis::Commands;
use tokio::main;
use rlua::Lua;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use std::fs;
use rusqlite::{params, Connection};

#[derive(Serialize, Deserialize)]
struct Event {
    agent_id: String,
    message: String,
    timestamp: u64,
}

use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result};

async fn admin_ui_proxy(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let client = reqwest::Client::new();
    let mut request = client.request(req.method().clone(), "http://localhost:2019".to_string() + req.uri().path())
        .body(body.to_vec());

    for (header_name, header_value) in req.headers() {
        request = request.header(header_name, header_value);
    }

    let response = request.send().await.unwrap();
    let mut client_resp = HttpResponse::build(response.status());
    for (header_name, header_value) in response.headers() {
        client_resp.append_header((header_name.clone(), header_value.clone()));
    }

    let body = response.bytes().await.unwrap();
    Ok(client_resp.body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let lua = Lua::new();
    let lua_script = fs::read_to_string("event_control.lua").expect("Failed to read Lua script");

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    // Subscribe to event stream
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("events").unwrap();

    // Listen to events
    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();

        let event: Event = serde_json::from_str(&payload).unwrap();

        lua.context(|ctx| {
            let globals = ctx.globals();
            globals.set("event", event).unwrap();

            ctx.load(&lua_script).exec().unwrap();

            let result: String = globals.get("result").unwrap();

            if result == "Process" {
                // Handle event processing
                process_event(event).await;
            }
        });
        HttpServer::new(|| {
            App::new()
                .default_service(web::route().to(admin_ui_proxy))
        })
        .bind("0.0.0.0:8080")?
        .run()
        .await;
    
    }
}

async fn admin_ui_proxy(req: HttpRequest, body: web::Bytes) -> Result<HttpResponse, actix_web::Error> {
    let client = reqwest::Client::new();
    let mut request = client.request(req.method().clone(), "http://localhost:2019".to_string() + req.uri().path())
        .body(body.to_vec());

    for (header_name, header_value) in req.headers() {
        request = request.header(header_name, header_value);
    }

    let response = request.send().await.unwrap();
    let mut client_resp = HttpResponse::build(response.status());
    for (header_name, header_value) in response.headers() {
        client_resp.append_header((header_name.clone(), header_value.clone()));
    }

    let body = response.bytes().await.unwrap();
    Ok(client_resp.body(body))
}

async fn process_event(event: Event) {
    // Your event processing logic here
    println!("Processing event: {:?}", event);

    // Example: Store event in SQLite
    let conn = Connection::open("events.db").unwrap();
    conn.execute(
        "INSERT INTO events (agent_id, message, timestamp) VALUES (?1, ?2, ?3)",
        params![event.agent_id, event.message, event.timestamp],
    ).unwrap();
}
