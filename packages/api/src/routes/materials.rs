use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use std::fs;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/materials", get(get_materials))
        .route("/api/materials/{category}", get(get_materials_by_category))
}

async fn get_materials(
    State(_state): State<AppState>,
) -> Json<serde_json::Value> {
    let data = fs::read_to_string("../data/materials.json")
        .expect("Failed to read materials.json");

    let materials: serde_json::Value = serde_json::from_str(&data)
        .expect("Failed to parse materials.json");

    Json(materials)
}

async fn get_materials_by_category(
    State(_state): State<AppState>,
    Path(category): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let data = fs::read_to_string("../data/materials.json")
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let materials: serde_json::Value = serde_json::from_str(&data)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(category_data) = materials.get(&category) {
        Ok(Json(category_data.clone()))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}
