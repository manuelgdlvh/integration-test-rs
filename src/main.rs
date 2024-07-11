use lib::http_server::HttpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mut server = HttpServer::build("0.0.0.0:3000", "postgres", "postgres", "postgres", "localhost", 5432).await?;
    server.start().await
}
