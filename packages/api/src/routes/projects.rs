use axum::{
    extract::{Path, State, Request},
    http::{StatusCode, HeaderValue},
    middleware::Next,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::auth;
use crate::schema;
use crate::utils::{ok, error_response};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/projects", get(list_projects).post(create_project))
        .route(
            "/api/projects/{id}",
            get(get_project).put(update_project).delete(delete_project),
        )
}

#[derive(Deserialize)]
pub struct CreateProject {
    pub plot_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProject {
    pub status: Option<String>,
    pub description: Option<String>,
}

async fn list_projects(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    let projects = sqlx::query_as!(
        schema::ProjectRow,
        "SELECT id, user_id, plot_id, status, description, created_at, updated_at FROM projects WHERE user_id = $1 ORDER BY created_at DESC",
        user_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let projects_json: Vec<serde_json::Value> = projects
        .iter()
        .map(|p| {
            serde_json::json!({
                "id": p.id,
                "user_id": p.user_id,
                "plot_id": p.plot_id,
                "status": p.status,
                "description": p.description,
                "created_at": p.created_at.to_rfc3339(),
                "updated_at": p.updated_at.to_rfc3339()
            })
        })
        .collect();

    Ok(Json(projects_json))
}

async fn create_project(
    State(state): State<AppState>,
    req: Request,
    next: Next,
    Json(payload): Json<CreateProject>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    let project = sqlx::query!(
        "INSERT INTO projects (user_id, plot_id, status, description) VALUES ($1, $2, 'draft', $3) RETURNING id, user_id, plot_id, status, description, created_at, updated_at",
        user_id,
        payload.plot_id,
        payload.description
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let project_json = serde_json::json!({
        "id": project.id,
        "user_id": project.user_id,
        "plot_id": project.plot_id,
        "status": project.status,
        "description": project.description,
        "created_at": project.created_at.to_rfc3339(),
        "updated_at": project.updated_at.to_rfc3339()
    });

    Ok((StatusCode::CREATED, Json(project_json)))
}

async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    let project = sqlx::query_as!(
        schema::ProjectRow,
        "SELECT id, user_id, plot_id, status, description, created_at, updated_at FROM projects WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match project {
        Some(p) => {
            let project_json = serde_json::json!({
                "id": p.id,
                "user_id": p.user_id,
                "plot_id": p.plot_id,
                "status": p.status,
                "description": p.description,
                "created_at": p.created_at.to_rfc3339(),
                "updated_at": p.updated_at.to_rfc3339()
            });
            Ok(Json(project_json))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    req: Request,
    next: Next,
    Json(payload): Json<UpdateProject>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    let project = sqlx::query!(
        "UPDATE projects SET status = COALESCE($1, status), description = COALESCE($2, description), updated_at = NOW() WHERE id = $3 AND user_id = $4 RETURNING id, user_id, plot_id, status, description, created_at, updated_at",
        payload.status,
        payload.description,
        id,
        user_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let project_json = serde_json::json!({
        "id": project.id,
        "user_id": project.user_id,
        "plot_id": project.plot_id,
        "status": project.status,
        "description": project.description,
        "created_at": project.created_at.to_rfc3339(),
        "updated_at": project.updated_at.to_rfc3339()
    });

    Ok(Json(project_json))
}

async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    let result = sqlx::query!("DELETE FROM projects WHERE id = $1 AND user_id = $2", id, user_id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(serde_json::json!({"message": "Project deleted"})))
}
