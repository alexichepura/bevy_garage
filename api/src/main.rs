use axum::{Extension, Router};
use db::PrismaClient;
use prisma_client_rust::NewClientError;
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
pub mod db;
pub mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    let prisma_client = Arc::new(client.unwrap());

    #[cfg(debug)]
    prisma_client._db_push(false).await.unwrap();

    let app = Router::new()
        .nest("/api", routes::create_route())
        .layer(Extension(prisma_client))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
