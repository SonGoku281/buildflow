use axum::{
    extract::{Path, State, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use uuid::Uuid;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/admin/projects", get(list_all_projects))
        .route("/api/admin/projects/{id}", get(get_admin_project))
        .route("/api/admin/leads", get(list_all_leads).put(update_lead_status))
        .route("/api/admin/feedback", get(get_feedback))
        .route("/api/admin/analytics", get(get_analytics))
        .route("/api/admin/estimates/{id}", put(update_estimate_status))
}

async fn list_all_projects(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let projects = sqlx::query!(
        "SELECT p.id, p.user_id, p.plot_id, p.status, p.description, p.created_at, p.updated_at, pr.email as user_email
         FROM projects p
         JOIN profiles pr ON pr.id = p.user_id
         ORDER BY p.created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json: Vec<serde_json::Value> = projects
        .iter()
        .map(|p| serde_json::json!({
            "id": p.id,
            "user_email": p.user_email,
            "status": p.status,
            "description": p.description,
            "created_at": p.created_at.to_rfc3339(),
            "updated_at": p.updated_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(json))
}

async fn get_admin_project(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let project = sqlx::query!(
        "SELECT p.id, p.user_id, p.plot_id, p.status, p.description, p.created_at, p.updated_at, pr.email as user_email
         FROM projects p
         JOIN profiles pr ON pr.id = p.user_id
         WHERE p.id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": project.id,
        "user_email": project.user_email,
        "status": project.status,
        "description": project.description,
        "created_at": project.created_at.to_rfc3339(),
        "updated_at": project.updated_at.to_rfc3339()
    })))
}

async fn list_all_leads(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let leads = sqlx::query!(
        "SELECT id, name, email, phone, district, project_type, budget_range, status, source, created_at FROM leads ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json: Vec<serde_json::Value> = leads
        .iter()
        .map(|l| serde_json::json!({
            "id": l.id,
            "name": l.name,
            "email": l.email,
            "phone": l.phone,
            "district": l.district,
            "status": l.status,
            "created_at": l.created_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(json))
}

async fn update_lead_status(
    State(state): State<AppState>,
    Json(status): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query!("UPDATE leads SET status = $1, updated_at = NOW() WHERE id = $2",
        status["status"].as_str().unwrap_or("new"),
        status["id"].as_str().unwrap_or("").parse::<Uuid>().unwrap_or(Uuid::nil())
    )
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({"message": "Lead status updated"})))
}

async fn get_feedback(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let feedback = sqlx::query!(
        "SELECT id, user_id, project_id, rating, comment, feature_interest, created_at FROM feedback ORDER BY created_at DESC LIMIT 100"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json: Vec<serde_json::Value> = feedback
        .iter()
        .map(|f| serde_json::json!({
            "id": f.id,
            "rating": f.rating,
            "comment": f.comment,
            "created_at": f.created_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(json))
}

async fn get_analytics(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    // Basic stats
    let total_projects = sqlx::query!("SELECT COUNT(*) as count FROM projects")
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total_leads = sqlx::query!("SELECT COUNT(*) as count FROM leads WHERE status = 'new'")
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let events_today = sqlx::query!(
        "SELECT event_name, COUNT(*) as count FROM analytics_events WHERE created_at >= NOW() - INTERVAL '24 hours' GROUP BY event_name"
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let json: Vec<serde_json::Value> = events_today
        .iter()
        .map(|e| serde_json::json!({
            "event_name": e.event_name,
            "count": e.count
        }))
        .collect();

    Ok(Json(serde_json::json!({
        "total_projects": total_projects.count,
        "new_leads": total_leads.count,
        "events_today": json
    })))
}

async fn update_estimate_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(status): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query!(
        "UPDATE estimates SET status = $1, updated_at = NOW() WHERE id = $2",
        status["status"].as_str().unwrap_or("pending"),
        id
    )
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({"message": "Estimate status updated"})))
}
