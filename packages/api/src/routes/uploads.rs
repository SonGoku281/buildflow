use axum::{
    extract::{Path, State, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/projects/{project_id}/uploads",
            get(list_uploads).post(upload_file),
        )
        .route("/api/uploads/{id}", delete(delete_upload))
}

async fn list_uploads(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let uploads = sqlx::query!(
        "SELECT id, user_id, project_id, file_url, file_type, description, created_at FROM uploads WHERE project_id = $1 ORDER BY created_at DESC",
        project_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json: Vec<serde_json::Value> = uploads
        .iter()
        .map(|u| serde_json::json!({
            "id": u.id,
            "file_url": u.file_url,
            "file_type": u.file_type,
            "description": u.description,
            "created_at": u.created_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(json))
}

async fn upload_file(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // Return Supabase presigned URL for actual upload
    // In production, generate presigned URL via Supabase Storage API
    Ok(Json(serde_json::json!({
        "message": "Upload to Supabase Storage bucket 'uploads'",
        "project_id": project_id,
        "presigned_url_placeholder": true
    })))
}

async fn delete_upload(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let _user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    sqlx::query!("DELETE FROM uploads WHERE id = $1", id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({"message": "Upload deleted"})))
}
