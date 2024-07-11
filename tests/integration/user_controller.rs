#[tokio::test]
async fn test() -> anyhow::Result<()> {
    let body = reqwest::get("http://localhost:3000/get")
        .await?
        .text()
        .await?;

    println!("Response: {}", body);
    Ok(())
}

#[tokio::test]
async fn test2() -> anyhow::Result<()> {
    let body = reqwest::get("http://localhost:3000/get")
        .await?
        .text()
        .await?;

    println!("Response: {}", body);
    Ok(())
}






