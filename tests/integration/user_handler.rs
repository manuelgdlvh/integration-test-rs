use reqwest::{Client, StatusCode};

#[tokio::test]
async fn should_create_user() -> anyhow::Result<()> {
    let request = r#"{"data": "data_test"}"#;

    let client = Client::new();
    let response = client.post("http://localhost:3000/user/create")
        .header("Content-Type", "application/json")
        .body(request)
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    let response_body = response.text().await?;
    assert_eq!(response_body.as_str(), r#"{"response":"data_test"}"#);
    Ok(())
}





