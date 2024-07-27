use lib::system::database_config::DatabaseConfig;
use lib::system::http_server::HttpServer;
use lib::system::server_config::ServerConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    
    let db_config = DatabaseConfig::new("postgres", "postgres", "postgres", "127.0.0.1", 5432);
    let server_config = ServerConfig::new("127.0.0.1", 3000);
    let mut http_server = HttpServer::build(db_config, server_config).await?;
    http_server.start().await
}
