use axum::{Router, routing::{get, put}};

pub fn router() -> Router {
    Router::new()
        .route("/api/admin/projects", get(list_all_projects))
        .route("/api/admin/projects/{id}", get(get_admin_project))
        .route("/api/admin/leads", get(list_all_leads).put(update_lead_status))
        .route("/api/admin/feedback", get(get_feedback))
        .route("/api/admin/analytics", get(get_analytics))
        .route("/api/admin/estimates/{id}", put(update_estimate_status))
}

async fn list_all_projects() -> &'static str { "GET - Admin only" }
async fn get_admin_project() -> &'static str { "GET - Admin only" }
async fn list_all_leads() -> &'static str { "GET - Admin only" }
async fn update_lead_status() -> &'static str { "PUT - Admin only" }
async fn get_feedback() -> &'static str { "GET - Admin only" }
async fn get_analytics() -> &'static str { "GET - Admin only" }
async fn update_estimate_status() -> &'static str { "PUT - Admin only" }
