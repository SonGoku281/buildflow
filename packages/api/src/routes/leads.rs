use axum::{Router, routing::{get, post, put}};

pub fn router() -> Router {
    Router::new()
        .route("/api/leads", get(list_leads).post(create_lead))
        .route("/api/leads/{id}", get(get_lead).put(update_lead))
}

async fn list_leads() -> &'static str { "GET - Admin only" }
async fn create_lead() -> &'static str { "POST - Public (anti-abuse: rate limit + OTP)" }
async fn get_lead() -> &'static str { "GET - Admin only" }
async fn update_lead() -> &'static str { "PUT - Admin only" }
