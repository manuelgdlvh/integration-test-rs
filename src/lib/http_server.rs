use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::handlers;
use crate::repositories::user_repository::{UserRepository, UserRepositoryImpl};
use crate::services::user_service::{UserService, UserServiceImpl};

pub struct HttpServer {
    listener: Option<TcpListener>,
    app: Option<Router>,
    started: bool,
}


impl HttpServer {
    pub async fn build(listener: &str, db_name: &str, db_user: &str, db_pass: &str, db_host: &str, db_port: u16) -> anyhow::Result<Self> {
        let db_pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(format!("postgres://{}:{}@{}:{}/{}", db_user, db_pass, db_host, db_port, db_name).as_str()).await?;
        let user_repository: UserRepository = Arc::new(UserRepositoryImpl::new(db_pool.clone()));
        let user_service: UserService = Arc::new(UserServiceImpl::new(user_repository));

        let app = Router::new()
            .route("/get", get(handlers::user_handler::create_user)).with_state(user_service);
        let listener = TcpListener::bind(listener).await?;

        Ok(Self {
            listener: Some(listener),
            app: Some(app),
            started: false,
        })
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        println!("starting web server...");
        if self.started {
            return Ok(());
        }

        let listener = self.listener.take().unwrap();
        let app = self.app.take().unwrap();
        self.started = true;

        axum::serve(listener, app).await?;
        Ok(())
    }
}