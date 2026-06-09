use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// SQLx query result types
#[derive(FromRow, Debug)]
pub struct ProjectRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plot_id: Option<Uuid>,
    pub status: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug)]
pub struct PlotRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub address: Option<String>,
    pub district: Option<String>,
    pub dimensions: serde_json::Value,
    pub area_sqft: Option<f64>,
    pub photos: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Debug)]
pub struct PreferenceRow {
    pub id: Uuid,
    pub project_id: Uuid,
    pub style: Option<String>,
    pub floors: Option<i32>,
    pub bhk: Option<String>,
    pub amenities: serde_json::Value,
    pub budget_min: Option<f64>,
    pub budget_max: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug)]
pub struct EstimateRow {
    pub id: Uuid,
    pub project_id: Uuid,
    pub version_number: i32,
    pub status: String,
    pub is_active: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug)]
pub struct EstimateVersionRow {
    pub id: Uuid,
    pub estimate_id: Uuid,
    pub version_number: i32,
    pub cost_breakdown: serde_json::Value,
    pub quality_tiers: serde_json::Value,
    pub additional_features: serde_json::Value,
    pub contingency_pct: Option<f64>,
    pub contingency_reasoning: Option<serde_json::Value>,
    pub layout_2d_url: Option<String>,
    pub layout_3d_url: Option<String>,
    pub bom_url: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Debug)]
pub struct LeadRow {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: String,
    pub district: Option<String>,
    pub project_type: Option<String>,
    pub budget_range: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Request/Response schemas
#[derive(Deserialize, Validate)]
pub struct CreateProfile {
    #[validate(length(min = 1))]
    pub email: String,
    #[validate(length(min = 10, max = 15))]
    pub phone: Option<String>,
    pub role: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct CreatePlot {
    pub address: Option<String>,
    pub district: Option<String>,
    pub dimensions: serde_json::Value,
    pub area_sqft: Option<f64>,
}

#[derive(Deserialize, Validate)]
pub struct CreateProject {
    pub plot_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProject {
    pub status: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plot_id: Option<Uuid>,
    pub status: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct CreatePreferences {
    pub style: Option<String>,
    pub floors: Option<i32>,
    pub bhk: Option<String>,
    pub amenities: Option<serde_json::Value>,
    pub budget_min: Option<f64>,
    pub budget_max: Option<f64>,
}

#[derive(Deserialize, Validate)]
pub struct CreateLead {
    #[validate(length(min = 1))]
    pub name: String,
    pub email: Option<String>,
    #[validate(length(min = 10, max = 15))]
    pub phone: String,
    pub district: Option<String>,
    pub project_type: Option<String>,
    pub budget_range: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize)]
pub struct LeadResponse {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: String,
    pub district: Option<String>,
    pub project_type: Option<String>,
    pub budget_range: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
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

#[derive(Serialize)]
pub struct EstimateResponse {
    pub id: Uuid,
    pub project_id: Uuid,
    pub version_number: i32,
    pub status: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct TrackEvent {
    pub event_name: String,
    pub event_data: Option<serde_json::Value>,
}
