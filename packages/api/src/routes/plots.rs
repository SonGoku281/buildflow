use axum::{
    extract::{Path, State, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::schema;
use crate::utils::ok;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/projects/{project_id}/plots",
            get(list_plots).post(create_plot),
        )
        .route(
            "/api/projects/{project_id}/plots/{id}",
            get(get_plot).put(update_plot).delete(delete_plot),
        )
}

#[derive(Deserialize)]
pub struct CreatePlot {
    pub address: Option<String>,
    pub district: Option<String>,
    pub dimensions: serde_json::Value,
    pub area_sqft: Option<f64>,
}

async fn list_plots(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let plots = sqlx::query_as!(
        schema::PlotRow,
        "SELECT id, user_id, address, district, dimensions, area_sqft, photos, created_at FROM plots WHERE id IN (SELECT plot_id FROM projects WHERE id = $1)",
        project_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let plots_json: Vec<serde_json::Value> = plots
        .iter()
        .map(|p| serde_json::json!({
            "id": p.id,
            "address": p.address,
            "district": p.district,
            "dimensions": p.dimensions,
            "area_sqft": p.area_sqft,
            "photos": p.photos,
            "created_at": p.created_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(plots_json))
}

async fn create_plot(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    req: Request,
    next: Next,
    Json(payload): Json<CreatePlot>,
) -> Result<impl IntoResponse, StatusCode> {
    // Get user_id from project
    let user_id = sqlx::query!("SELECT user_id FROM projects WHERE id = $1", project_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?
        .user_id;

    let plot = sqlx::query!(
        "INSERT INTO plots (user_id, address, district, dimensions, area_sqft) VALUES ($1, $2, $3, $4, $5) RETURNING id, user_id, address, district, dimensions, area_sqft, photos, created_at",
        user_id,
        payload.address,
        payload.district,
        payload.dimensions,
        payload.area_sqft
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": plot.id,
        "user_id": plot.user_id,
        "address": plot.address,
        "district": plot.district,
        "dimensions": plot.dimensions,
        "area_sqft": plot.area_sqft,
        "photos": plot.photos,
        "created_at": plot.created_at.to_rfc3339()
    })))
}

async fn get_plot(
    State(state): State<AppState>,
    Path((project_id, plot_id)): Path<(Uuid, Uuid)>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let plot = sqlx::query_as!(
        schema::PlotRow,
        "SELECT id, user_id, address, district, dimensions, area_sqft, photos, created_at FROM plots WHERE id = $1",
        plot_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": plot.id,
        "address": plot.address,
        "district": plot.district,
        "dimensions": plot.dimensions,
        "area_sqft": plot.area_sqft,
        "photos": plot.photos,
        "created_at": plot.created_at.to_rfc3339()
    })))
}

async fn update_plot(
    State(state): State<AppState>,
    Path((project_id, plot_id)): Path<(Uuid, Uuid)>,
    req: Request,
    next: Next,
    Json(payload): Json<CreatePlot>,
) -> Result<impl IntoResponse, StatusCode> {
    let plot = sqlx::query!(
        "UPDATE plots SET address = COALESCE($1, address), district = COALESCE($2, district), dimensions = COALESCE($3, dimensions), area_sqft = COALESCE($4, area_sqft), updated_at = NOW() WHERE id = $5 RETURNING id, user_id, address, district, dimensions, area_sqft, photos, created_at",
        payload.address,
        payload.district,
        payload.dimensions,
        payload.area_sqft,
        plot_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": plot.id,
        "address": plot.address,
        "district": plot.district,
        "dimensions": plot.dimensions,
        "area_sqft": plot.area_sqft,
        "photos": plot.photos,
        "created_at": plot.created_at.to_rfc3339()
    })))
}

async fn delete_plot(
    State(state): State<AppState>,
    Path((project_id, plot_id)): Path<(Uuid, Uuid)>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let result = sqlx::query!("DELETE FROM plots WHERE id = $1", plot_id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(serde_json::json!({"message": "Plot deleted"})))
}
