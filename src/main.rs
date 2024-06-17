mod auth;
mod routes;
mod utils;

use auth::auth;
use axum::routing::{get, patch, post};
use axum::{middleware, Router};
use axum_prometheus::PrometheusMetricLayer;
use dotenvy::dotenv;
use routes::{create_link, get_link_statistics, redirect, update_link};
use sqlx::postgres::PgPoolOptions;
use std::error::Error;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::routes::health;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "link_shortner=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required env variable");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Could not connect to database");

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/create", post(create_link))
        .route("/:id/statistics", get(get_link_statistics))
        .route_layer(middleware::from_fn_with_state(db.clone(), auth))
        .route(
            "/:id",
            patch(update_link)
                .route_layer(middleware::from_fn_with_state(db.clone(), auth))
                .get(redirect),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .with_state(db);

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not initialize TcpListner");

    tracing::debug!(
        "Listening on {}",
        listner
            .local_addr()
            .expect("Could not convert listner address to local address")
    );

    axum::serve(listner, app)
        .await
        .expect("Could not crate server");

    Ok(())
}
