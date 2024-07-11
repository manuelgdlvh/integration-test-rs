use std::thread;
use std::time::Duration;

use ctor::{ctor, dtor};
use lazy_static::lazy_static;
use testcontainers::{Container, GenericImage};
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use tokio::runtime;

use lib::http_server::HttpServer;

lazy_static! {
    pub static ref DOCKER_CLI: Cli = {
             Cli::default()
    };
    pub static ref POSTGRES_CONTAINER: Container<'static, GenericImage> = {
        let container = GenericImage::new("postgres", "16.3")
            .with_env_var("POSTGRES_USER","postgres")
            .with_env_var("POSTGRES_PASSWORD","postgres")
            .with_env_var("POSTGRES_DB","postgres")
            .with_exposed_port(5432)
            .with_wait_for(WaitFor::message_on_stdout("database system is ready to accept connections"));
        DOCKER_CLI.run(container)
    };
}


#[ctor]
fn init() {
    thread::spawn(|| {
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let postgres_container = &*POSTGRES_CONTAINER;
                let mut http_server = HttpServer::build("127.0.0.1:3000", "postgres", "postgres", "postgres", "localhost"
                                                        , postgres_container.get_host_port_ipv4(5432)).await.unwrap();
                let _ = http_server.start().await;
            });
    });

    thread::sleep(Duration::new(5, 0));
}


#[dtor]
fn destroy() {
    (&*POSTGRES_CONTAINER).stop();
}





