use axum::{Router, routing::{post, get}};

pub fn router() -> Router {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/verify-otp", post(verify_otp))
        .route("/api/auth/me", get(get_me))
}

async fn login() -> &'static str {
    "POST /api/auth/login - Supabase JWT exchange"
}

async fn verify_otp() -> &'static str {
    "POST /api/auth/verify-otp - Phone OTP verification via Supabase"
}

async fn get_me() -> &'static str {
    "GET /api/auth/me - Returns authenticated user profile"
}
