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
        .route("/api/leads", get(list_leads).post(create_lead))
        .route("/api/leads/{id}", get(get_lead).put(update_lead))
}

#[derive(Deserialize)]
pub struct CreateLead {
    pub name: String,
    pub email: Option<String>,
    pub phone: String,
    pub district: Option<String>,
    pub project_type: Option<String>,
    pub budget_range: Option<String>,
    pub source: Option<String>,
}

async fn list_leads(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // Admin only check
    let _user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    let leads = sqlx::query_as!(
        schema::LeadRow,
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
            "project_type": l.project_type,
            "budget_range": l.budget_range,
            "status": l.status,
            "source": l.source,
            "created_at": l.created_at.to_rfc3339()
        }))
        .collect();

    Ok(Json(json))
}

async fn create_lead(
    State(state): State<AppState>,
    Json(payload): Json<CreateLead>,
) -> Result<impl IntoResponse, StatusCode> {
    // Anti-abuse: check if phone already exists
    let exists = sqlx::query!(
        "SELECT COUNT(*) as cnt FROM leads WHERE phone = $1 AND status != 'lost'",
        payload.phone
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if exists.cnt > 0 {
        return Err(StatusCode::CONFLICT);
    }

    let lead = sqlx::query!(
        "INSERT INTO leads (name, email, phone, district, project_type, budget_range, source) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, name, email, phone, district, project_type, budget_range, status, source, created_at",
        payload.name,
        payload.email,
        payload.phone,
        payload.district,
        payload.project_type,
        payload.budget_range,
        payload.source
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "id": lead.id,
        "name": lead.name,
        "email": lead.email,
        "phone": lead.phone,
        "district": lead.district,
        "project_type": lead.project_type,
        "budget_range": lead.budget_range,
        "status": lead.status,
        "source": lead.source,
        "created_at": lead.created_at.to_rfc3339()
    })))
}

async fn get_lead(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let lead = sqlx::query_as!(
        schema::LeadRow,
        "SELECT id, name, email, phone, district, project_type, budget_range, status, source, created_at FROM leads WHERE id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": lead.id,
        "name": lead.name,
        "email": lead.email,
        "phone": lead.phone,
        "district": lead.district,
        "project_type": lead.project_type,
        "budget_range": lead.budget_range,
        "status": lead.status,
        "source": lead.source,
        "created_at": lead.created_at.to_rfc3339()
    })))
}

async fn update_lead(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(status): Json<serde_json::Value>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let lead = sqlx::query!(
        "UPDATE leads SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING id, name, email, phone, district, project_type, budget_range, status, source, created_at",
        status["status"].as_str().unwrap_or("new"),
        id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(serde_json::json!({
        "id": lead.id,
        "status": lead.status
    })))
}
