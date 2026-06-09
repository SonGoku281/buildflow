use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod config;
mod middleware;
mod routes;
mod schema;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "buildflow_api=debug,axum=info,rustls=warn".into())
        )
        .init();

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/docs", get(utoipa_swagger_ui::SwaggerUi::new("/api/docs")))
        .merge(routes::auth::router())
        .merge(routes::projects::router())
        .merge(routes::plots::router())
        .merge(routes::preferences::router())
        .merge(routes::estimates::router())
        .merge(routes::leads::router())
        .merge(routes::uploads::router())
        .merge(routes::admin::router())
        .merge(routes::materials::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let port = config::get().port;
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("🚀 BuildFlow API starting on {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "ok"
}
