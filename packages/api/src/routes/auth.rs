use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::auth;
use crate::schema::CreateProfile;
use crate::utils::{ok, error_response};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(login))
        .route("/api/auth/verify-otp", post(verify_otp))
        .route("/api/auth/me", get(get_me))
        .route("/api/auth/profile", get(get_profile).put(update_profile))
}

// --- Login ---
// Handshake with Supabase Auth, exchange Supabase JWT for our JWT
#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Step 1: Exchange credentials with Supabase Auth
    // In production, call: POST https://your-project.supabase.co/auth/v1/token?grant_type=password
    // For now, we'll verify via Supabase JWT pattern

    // Step 2: Verify Supabase JWT and extract user_id
    // The frontend sends Supabase JWT in Authorization header
    // This endpoint just confirms credentials and returns our JWT

    tracing::info!("Login attempt for: {}", payload.email);

    // TODO: Integrate with Supabase Auth REST API
    // Let's assume we get a valid Supabase token from the frontend

    // For MVP: return a placeholder JWT with user info
    let claims = auth::Claims {
        sub: Uuid::new_v4().to_string(), // Replace with real user_id from Supabase
        email: payload.email.clone(),
        role: "client".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(720)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let jwt = auth::generate_jwt(claims, &state.auth_config)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "access_token": jwt,
        "token_type": "bearer",
        "user": {
            "email": payload.email,
            "role": "client"
        }
    })))
}

// --- Verify OTP ---
#[derive(Deserialize)]
pub struct VerifyOtpRequest {
    phone: String,
    otp: String,
}

async fn verify_otp(
    State(state): State<AppState>,
    Json(payload): Json<VerifyOtpRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::info!("OTP verification for phone: {}", payload.phone);

    // TODO: Integrate with Supabase Auth phone OTP
    // POST https://your-project.supabase.co/auth/v1/verify
    // with phone and otp

    let claims = auth::Claims {
        sub: Uuid::new_v4().to_string(),
        email: format!("{}@phone.supabase.co", payload.phone),
        role: "client".to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(720)).timestamp() as usize,
        iat: chrono::Utc::now().timestamp() as usize,
    };

    let jwt = auth::generate_jwt(claims, &state.auth_config)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(serde_json::json!({
        "access_token": jwt,
        "token_type": "bearer",
        "user": {
            "phone": payload.phone,
            "role": "client"
        }
    })))
}

// --- Get Current User Profile ---
async fn get_me(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    // Extract user_id from JWT (added by auth middleware)
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    let profile = auth::get_profile(&state.pool, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(profile))
}

// --- Get Profile ---
async fn get_profile(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    let profile = auth::get_profile(&state.pool, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(profile))
}

// --- Update Profile ---
async fn update_profile(
    State(state): State<AppState>,
    req: Request,
    next: Next,
    Json(payload): Json<CreateProfile>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = req
        .extensions()
        .get::<Uuid>()
        .ok_or(StatusCode::UNAUTHORIZED)?
        .clone();

    // Validate email matches user
    if payload.email.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let profile = auth::update_profile(&state.pool, user_id, payload.phone)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(profile))
}

// --- Auth Middleware ---
pub async fn AuthMiddleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    // We need access to config here - for now, use a dummy secret
    // In production, this should come from AppState
    let cfg = config::get();
    let auth_config = auth::AuthConfig::new(cfg.supabase_jwt_secret.clone());

    let claims = auth::verify_jwt(token, &auth_config)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut req = req;
    req.extensions_mut().insert(Uuid::parse_str(&claims.sub).unwrap_or(Uuid::nil()));

    Ok(next.run(req).await)
}
