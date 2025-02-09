#[cfg(test)]
mod tests {
    use super::*;
        use actix_rt::time::sleep;
            use std::time::Duration;

                #[actix_rt::test]
                    async fn test_mock_endpoint() {
                            let _ = run_mock_server().await;
                                    sleep(Duration::from_secs(1)).await; // Wait for the server to start

                                            let client = reqwest::Client::new();
                                                    let resp = client.get("http://127.0.0.1:8081/mock/event").send().await.unwrap();
                                                            assert_eq!(resp.status(), 200);
                                                                    let body = resp.text().await.unwrap();
                                                                            assert_eq!(body, "Mock response");
                                                                                }
                                                                                }