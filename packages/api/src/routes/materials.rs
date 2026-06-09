use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/api/materials", get(get_materials))
        .route("/api/materials/{category}", get(get_materials_by_category))
}

async fn get_materials() -> &'static str {
    "GET all materials from JSON catalog (read-only)"
}

async fn get_materials_by_category() -> &'static str {
    "GET materials filtered by category"
}
