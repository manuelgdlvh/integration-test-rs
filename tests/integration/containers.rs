use std::sync::{LazyLock, mpsc};
use std::thread;
use std::time::Duration;

use ctor::{ctor, dtor};
use testcontainers::{Container, GenericImage};
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use tokio::runtime;
use tokio::sync::oneshot;

use lib::system::database_config::DatabaseConfig;
use lib::system::http_server::HttpServer;
use lib::system::server_config::ServerConfig;

static DOCKER_CLI: LazyLock<Cli> = LazyLock::new(|| {
    Cli::default()
});

static POSTGRES_CONTAINER: LazyLock<Container<'static, GenericImage>> = LazyLock::new(|| {
    let container = GenericImage::new("postgres", "16.3")
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .with_env_var("POSTGRES_DB", "postgres")
        .with_exposed_port(5432)
        .with_wait_for(WaitFor::message_on_stdout("database system is ready to accept connections"));
    DOCKER_CLI.run(container)
});


#[ctor]
fn init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let postgres_container = &*POSTGRES_CONTAINER;

                let db_config = DatabaseConfig::new("postgres", "postgres", "postgres", "127.0.0.1", postgres_container.get_host_port_ipv4(5432));
                let server_config = ServerConfig::new("127.0.0.1", 3000);
                let mut http_server = HttpServer::build(db_config, server_config).await.unwrap();

                tx.send(()).unwrap();
                http_server.start().await
            });
    });

    log::info!("waiting to web server to be fully initialized...");
    rx.blocking_recv().unwrap();
}


#[dtor]
fn destroy() {
    (&*POSTGRES_CONTAINER).stop();
}





