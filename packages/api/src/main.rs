use axum::{
    extract::{State, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    Router,
    routing::{get, post, put, delete},
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod auth;
mod config;
mod middleware;
mod routes;
mod schema;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth_config: auth::AuthConfig,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "buildflow_api=debug,axum=info,rustls=warn".into())
        )
        .init();

    let cfg = config::get();

    let pool = auth::init_pool(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    // Test connection
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("Database connection test failed");

    tracing::info!("Database connection verified");

    let state = AppState {
        pool,
        auth_config: auth::AuthConfig::new(cfg.supabase_jwt_secret.clone()),
    };

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
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port = cfg.port;
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
