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
            "/api/projects/{project_id}/estimates",
            get(list_estimates).post(create_estimate),
        )
        .route(
            "/api/projects/{project_id}/estimates/{id}",
            get(get_estimate).put(update_estimate),
        )
        .route("/api/projects/{project_id}/estimates/{id}/versions", get(list_versions))
        .route("/api/estimates/{id}/request-revision", post(request_revision))
}

#[derive(Deserialize)]
pub struct CreateEstimate {
    pub quality_tiers: serde_json::Value,
    pub cost_breakdown: serde_json::Value,
    pub additional_features: Option<serde_json::Value>,
    pub contingency_pct: Option<f64>,
    pub contingency_reasoning: Option<serde_json::Value>,
    pub layout_2d_url: Option<String>,
    pub layout_3d_url: Option<String>,
    pub bom_url: Option<String>,
    pub notes: Option<String>,
}

async fn list_estimates(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let estimates = sqlx::query_as!(
        schema::EstimateRow,
        "SELECT id, project_id, version_number, status, is_active, created_by, created_at, updated_at FROM estimates WHERE project_id = $1 ORDER BY version_number DESC",
        project_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json: Vec<serde_json::Value> = estimates
        .iter()
        .map(|e| serde_json::json!({
            "id": e.id,
            "project_id": e.project_id,
            "version_number": e.version_number,
            "status": e.status,
            "is_active": e.is_active,
            "created_by": e.created_by,
            "created_at": e.created_at.to_rfc3339(),
            "updated_at": e.updated_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(json))
}

async fn create_estimate(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
    Json(payload): Json<CreateEstimate>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get next version number
    let next_version = sqlx::query!(
        "SELECT COALESCE(MAX(version_number), 0) + 1 as next FROM estimates WHERE project_id = $1",
        project_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    // Create estimate
    let estimate = sqlx::query!(
        "INSERT INTO estimates (project_id, version_number, status, created_by) VALUES ($1, $2, 'draft', $3) RETURNING id, project_id, version_number, status, is_active, created_by, created_at, updated_at",
        project_id,
        next_version.next,
        user_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create version snapshot
    sqlx::query!(
        "INSERT INTO estimates_versions (estimate_id, version_number, cost_breakdown, quality_tiers, additional_features, contingency_pct, contingency_reasoning, layout_2d_url, layout_3d_url, bom_url, notes) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        estimate.id,
        next_version.next,
        payload.cost_breakdown,
        payload.quality_tiers,
        payload.additional_features.unwrap_or(serde_json::json!([])),
        payload.contingency_pct,
        payload.contingency_reasoning,
        payload.layout_2d_url,
        payload.layout_3d_url,
        payload.bom_url,
        payload.notes
    )
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": estimate.id,
        "project_id": estimate.project_id,
        "version_number": estimate.version_number,
        "status": estimate.status,
        "created_at": estimate.created_at.to_rfc3339(),
        "updated_at": estimate.updated_at.to_rfc3339()
    })))
}

async fn get_estimate(
    State(state): State<AppState>,
    Path((project_id, id)): Path<(Uuid, Uuid)>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let estimate = sqlx::query_as!(
        schema::EstimateRow,
        "SELECT id, project_id, version_number, status, is_active, created_by, created_at, updated_at FROM estimates WHERE id = $1 AND project_id = $2",
        id,
        project_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let versions = sqlx::query_as!(
        schema::EstimateVersionRow,
        "SELECT id, estimate_id, version_number, cost_breakdown, quality_tiers, additional_features, contingency_pct, contingency_reasoning, layout_2d_url, layout_3d_url, bom_url, notes, created_at FROM estimates_versions WHERE estimate_id = $1 ORDER BY version_number ASC",
        id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let versions_json: Vec<serde_json::Value> = versions
        .iter()
        .map(|v| serde_json::json!({
            "id": v.id,
            "version_number": v.version_number,
            "cost_breakdown": v.cost_breakdown,
            "quality_tiers": v.quality_tiers,
            "additional_features": v.additional_features,
            "contingency_pct": v.contingency_pct,
            "contingency_reasoning": v.contingency_reasoning,
            "layout_2d_url": v.layout_2d_url,
            "layout_3d_url": v.layout_3d_url,
            "bom_url": v.bom_url,
            "notes": v.notes,
            "created_at": v.created_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(serde_json::json!({
        "id": estimate.id,
        "project_id": estimate.project_id,
        "version_number": estimate.version_number,
        "status": estimate.status,
        "is_active": estimate.is_active,
        "created_by": estimate.created_by,
        "created_at": estimate.created_at.to_rfc3339(),
        "updated_at": estimate.updated_at.to_rfc3339(),
        "versions": versions_json
    })))
}

async fn update_estimate(
    State(state): State<AppState>,
    Path((project_id, id)): Path<(Uuid, Uuid)>,
    req: Request,
    next: Next,
    Json(status): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    let estimate = sqlx::query!(
        "UPDATE estimates SET status = $1, updated_at = NOW() WHERE id = $2 AND project_id = $3 RETURNING id, project_id, version_number, status, is_active, created_by, created_at, updated_at",
        status["status"].as_str().unwrap_or("draft"),
        id,
        project_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": estimate.id,
        "status": estimate.status,
        "updated_at": estimate.updated_at.to_rfc3339()
    })))
}

async fn list_versions(
    State(state): State<AppState>,
    Path((project_id, id)): Path<(Uuid, Uuid)>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let versions = sqlx::query_as!(
        schema::EstimateVersionRow,
        "SELECT id, estimate_id, version_number, cost_breakdown, quality_tiers, additional_features, contingency_pct, contingency_reasoning, layout_2d_url, layout_3d_url, bom_url, notes, created_at FROM estimates_versions WHERE estimate_id = $1 ORDER BY version_number ASC",
        id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json: Vec<serde_json::Value> = versions
        .iter()
        .map(|v| serde_json::json!({
            "version_number": v.version_number,
            "cost_breakdown": v.cost_breakdown,
            "quality_tiers": v.quality_tiers,
            "contingency_pct": v.contingency_pct,
            "created_at": v.created_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(json))
}

async fn request_revision(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query!(
        "UPDATE estimates SET status = 'revising', updated_at = NOW() WHERE id = $1",
        id
    )
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({"message": "Revision requested"})))
}
