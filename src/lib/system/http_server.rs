use std::convert::Infallible;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::process::Output;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::{async_trait, BoxError, middleware, RequestExt, Router, routing::{get, post}};
use axum::body::{Body, Bytes, to_bytes};
use axum::extract::{FromRequest, Request};
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post_service;
use axum_extra::handler::HandlerCallWithExtractors;
use serde::{de, Deserialize, Deserializer};
use serde::de::DeserializeOwned;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio::join;
use tokio::net::TcpListener;

use crate::handlers;

use super::{database_config::DatabaseConfig, server_config::ServerConfig};

pub struct HttpServer {
    app: Option<Router>,
    listener: Option<TcpListener>,
    started: bool,
}

impl HttpServer {
    pub async fn build(
        db_config: DatabaseConfig<'_>,
        server_config: ServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        let db_pool = Self::open_database(db_config).await?;

        let user_routes = Router::new()
            .route_service("/create", post(handlers::user_handler::create_user));
        let routes = Router::new().nest("/user", user_routes);

        Ok(Self {
            app: Some(routes),
            listener: Some(
                TcpListener::bind(format!("{}:{}", server_config.host(), server_config.port())).await?,
            ),
            started: false,
        })
    }


    async fn open_database(db_config: DatabaseConfig<'_>) -> anyhow::Result<Pool<Postgres>> {
        log::info!("initializing database connection pool...");

        let db_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(
                format!(
                    "postgres://{}:{}@{}:{}/{}",
                    db_config.username(),
                    db_config.password(),
                    db_config.host(),
                    db_config.port(),
                    db_config.db_name()
                )
                    .as_str(),
            )
            .await?;

        log::info!("initialized database connection pool successfully");

        Ok(db_pool)
    }


    pub async fn start(&mut self) -> anyhow::Result<()> {
        log::info!("http server initializing for listen incoming requests...");
        if self.started {
            return Ok(());
        }
        self.started = true;
        let app = self.app.take().unwrap();
        let listener = self.listener.take().unwrap();
        axum::serve(listener, app).await.unwrap();
        Ok(())
    }
}


