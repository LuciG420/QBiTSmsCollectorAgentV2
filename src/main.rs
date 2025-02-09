mod events {
        include!(concat!(env!("OUT_DIR"), "/events.rs"));
        }

        use events::{Event, MetadataEvent, EventBatch};
        use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result};
        use tokio_tungstenite::connect_async;
        use tokio_tungstenite::tungstenite::protocol::Message;
        use hyper::{Client, Uri};
        use hyper::client::HttpConnector;
        use rusqlite::{params, Connection};
        use std::sync::{Arc, Mutex};
        use std::time::{SystemTime, UNIX_EPOCH, Duration};
        use std::collections::HashMap;
        use futures_util::{SinkExt, StreamExt};
        use tokio_retry::{Retry, strategy::ExponentialBackoff};
        use aws_sdk_s3::{Client as S3Client, Region};
        use aws_sdk_s3::types::SdkError;

        async fn upload_to_s3(client: &S3Client, bucket: &str, key: &str, body: Vec<u8>) -> Result<(), SdkError<aws_sdk_s3::error::PutObjectError>> {
            let req = client.put_object()
                    .bucket(bucket)
                            .key(key)
                                    .body(body.into())
                                            .send().await;

                                                req.map(|_| ()).map_err(SdkError::from)
                                                }

                                                async fn export_and_upload_events() -> Result<(), Box<dyn std::error::Error>> {
                                                    let conn = Connection::open("events.db")?;
                                                        let mut stmt = conn.prepare("SELECT chat_session_id, message, timestamp FROM events")?;
                                                            let events = stmt.query_map([], |row| {
                                                                    Ok(Event {
                                                                                chat_session_id: row.get(0)?,
                                                                                            message: row.get(1)?,
                                                                                                        timestamp: row.get(2)?,
                                                                                                                })
                                                                                                                    })?;

                                                                                                                        let schema = Arc::new(
                                                                                                                                Type::group_type_builder("schema")
                                                                                                                                            .with_fields(&mut vec![
                                                                                                                                                            Type::primitive_type_builder("chat_session_id", PhysicalType::BYTE_ARRAY).build().unwrap(),
                                                                                                                                                                            Type::primitive_type_builder("message", PhysicalType::BYTE_ARRAY).build().unwrap(),
                                                                                                                                                                                            Type::primitive_type_builder("timestamp", PhysicalType::INT64).build().unwrap(),
                                                                                                                                                                                                        ])
                                                                                                                                                                                                                    .build()
                                                                                                                                                                                                                                .unwrap(),
                                                                                                                                                                                                                                    );

                                                                                                                                                                                                                                        let file = File::create("events.parquet")?;
                                                                                                                                                                                                                                            let mut writer = SerializedFileWriter::new(file, schema)?;

                                                                                                                                                                                                                                                for event in events {
                                                                                                                                                                                                                                                        let event = event?;
                                                                                                                                                                                                                                                                // Logic to write data to Parquet file
                                                                                                                                                                                                                                                                    }

                                                                                                                                                                                                                                                                        let s3_client = S3Client::new(&Region::new("nyc3")); // Region specific to DigitalOcean Spaces
                                                                                                                                                                                                                                                                            let retry_strategy = ExponentialBackoff::from_millis(10).take(5); // Retry strategy

                                                                                                                                                                                                                                                                                let upload_result = Retry::spawn(retry_strategy, || upload_to_s3(&s3_client, "your-bucket", "events.parquet", std::fs::read("events.parquet")?)).await;

                                                                                                                                                                                                                                                                                    match upload_result {
                                                                                                                                                                                                                                                                                            Ok(_) => println!("Successfully uploaded Parquet file to S3."),
                                                                                                                                                                                                                                                                                                    Err(e) => println!("Failed to upload Parquet file to S3: {:?}", e),
                                                                                                                                                                                                                                                                                                        }

                                                                                                                                                                                                                                                                                                            // Delete old records from the database
                                                                                                                                                                                                                                                                                                                conn.execute("DELETE FROM events WHERE timestamp < ?", params![(SystemTime::now() - Duration::new(30 * 24 * 60 * 60, 0)).duration_since(UNIX_EPOCH).unwrap().as_secs()])?;

                                                                                                                                                                                                                                                                                                                    Ok(())
                                                                                                                                                                                                                                                                                                                    }

                                                                                                                                                                                                                                                                                                                    #[actix_web::main]
                                                                                                                                                                                                                                                                                                                    async fn main() -> std::io::Result<()> {
                                                                                                                                                                                                                                                                                                                        export_and_upload_events().await.unwrap();

                                                                                                                                                                                                                                                                                                                            HttpServer::new(|| {
                                                                                                                                                                                                                                                                                                                                    App::new()
                                                                                                                                                                                                                                                                                                                                                .route("/ws/", web::get().to(proxy_http2_to_ably))
                                                                                                                                                                                                                                                                                                                                                    })
                                                                                                                                                                                                                                                                                                                                                        .bind("0.0.0.0:8080")?
                                                                                                                                                                                                                                                                                                                                                            .run()
                                                                                                                                                                                                                                                                                                                                                                .await
                                                                                                                                                                                                                                                                                                                                                                } 
}