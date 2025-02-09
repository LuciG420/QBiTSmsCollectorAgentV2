use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct AdminRequest {
    method: String,
    url: String,
    body: Option<String>,
}

struct WebSocketProxy;

impl actix::Actor for WebSocketProxy {
    type Context = ws::WebsocketContext<Self>;
}

impl ws::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketProxy {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Forward the message to the central management application
                let req: AdminRequest = serde_json::from_str(&text).unwrap();
                tokio::spawn(async move {
                    let (mut write, mut read) = connect_async("ws://central-management-app:8080").await.unwrap().split();
                    write.send(tokio_tungstenite::tungstenite::protocol::Message::Text(text)).await.unwrap();
                });
            }
            _ => (),
        }
    }
}

async fn proxy(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let (response, mut session, mut stream) = ws::start_with_protocols(WebSocketProxy, &["rust-proxy"], &req, stream)?;
    session.text("Proxy connection established");

    tokio::spawn(async move {
        while let Some(msg) = stream.next().await {
            if let Ok(ws::Message::Text(text)) = msg {
                // Handle incoming messages from the client
                // Forward to central management application
                let req: AdminRequest = serde_json::from_str(&text).unwrap();
                let client = reqwest::Client::new();
                let response = client.request(req.method.parse().unwrap(), &req.url)
                    .body(req.body.unwrap_or_default())
                    .send()
                    .await
                    .unwrap();
                let response_text = response.text().await.unwrap();
                stream.send(ws::Message::Text(response_text)).await.unwrap();
            }
        }
    });

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/ws/", web::get().to(proxy))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

