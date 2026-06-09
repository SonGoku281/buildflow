use axum::{
    extract::{Path, State, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::schema;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/projects/{project_id}/preferences",
            get(get_preferences).post(set_preferences).put(update_preferences),
        )
}

#[derive(Deserialize)]
pub struct CreatePreferences {
    pub style: Option<String>,
    pub floors: Option<i32>,
    pub bhk: Option<String>,
    pub amenities: Option<serde_json::Value>,
    pub budget_min: Option<f64>,
    pub budget_max: Option<f64>,
}

async fn get_preferences(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let pref = sqlx::query_as!(
        schema::PreferenceRow,
        "SELECT id, project_id, style, floors, bhk, amenities, budget_min, budget_max, created_at, updated_at FROM preferences WHERE project_id = $1",
        project_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match pref {
        Some(p) => Ok(Json(serde_json::json!({
            "id": p.id,
            "project_id": p.project_id,
            "style": p.style,
            "floors": p.floors,
            "bhk": p.bhk,
            "amenities": p.amenities,
            "budget_min": p.budget_min,
            "budget_max": p.budget_max,
            "created_at": p.created_at.to_rfc3339(),
            "updated_at": p.updated_at.to_rfc3339()
        }))),
        None => Ok(Json(serde_json::json!({}))),
    }
}

async fn set_preferences(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
    Json(payload): Json<CreatePreferences>,
) -> Result<impl IntoResponse, StatusCode> {
    let pref = sqlx::query!(
        "INSERT INTO preferences (project_id, style, floors, bhk, amenities, budget_min, budget_max) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, project_id, style, floors, bhk, amenities, budget_min, budget_max, created_at, updated_at",
        project_id,
        payload.style,
        payload.floors,
        payload.bhk,
        payload.amenities.unwrap_or(serde_json::json!([])),
        payload.budget_min,
        payload.budget_max
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": pref.id,
        "project_id": pref.project_id,
        "style": pref.style,
        "floors": pref.floors,
        "bhk": pref.bhk,
        "amenities": pref.amenities,
        "budget_min": pref.budget_min,
        "budget_max": pref.budget_max,
        "created_at": pref.created_at.to_rfc3339(),
        "updated_at": pref.updated_at.to_rfc3339()
    })))
}

async fn update_preferences(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
    Json(payload): Json<CreatePreferences>,
) -> Result<impl IntoResponse, StatusCode> {
    let pref = sqlx::query!(
        "UPDATE preferences SET style = COALESCE($1, style), floors = COALESCE($2, floors), bhk = COALESCE($3, bhk), amenities = COALESCE($4, amenities), budget_min = COALESCE($5, budget_min), budget_max = COALESCE($6, budget_max), updated_at = NOW() WHERE project_id = $7 RETURNING id, project_id, style, floors, bhk, amenities, budget_min, budget_max, created_at, updated_at",
        payload.style,
        payload.floors,
        payload.bhk,
        payload.amenities.unwrap_or(serde_json::json!([])),
        payload.budget_min,
        payload.budget_max,
        project_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": pref.id,
        "project_id": pref.project_id,
        "style": pref.style,
        "floors": pref.floors,
        "bhk": pref.bhk,
        "amenities": pref.amenities,
        "budget_min": pref.budget_min,
        "budget_max": pref.budget_max,
        "created_at": pref.created_at.to_rfc3339(),
        "updated_at": pref.updated_at.to_rfc3339()
    })))
}
