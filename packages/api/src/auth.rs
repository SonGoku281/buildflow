use sqlx::PgPool;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // user ID
    pub email: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: u64,
}

impl AuthConfig {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret,
            token_expiry_hours: 720, // 30 days
        }
    }
}

pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}

pub fn generate_jwt(claims: Claims, config: &AuthConfig) -> Result<String, String> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| e.to_string())
}

pub fn verify_jwt(token: &str, config: &AuthConfig) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .map_err(|e| e.to_string())
}

pub async fn get_profile(pool: &PgPool, user_id: Uuid) -> Result<serde_json::Value, String> {
    let row = sqlx::query("SELECT get_user_profile($1)")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    match row {
        Some(row) => {
            let id: Uuid = row.get("id");
            let email: String = row.get("email");
            let phone: Option<String> = row.get("phone");
            let role: String = row.get("role");
            let avatar_url: Option<String> = row.get("avatar_url");
            let created_at: chrono::DateTime<Utc> = row.get("created_at");

            Ok(serde_json::json!({
                "id": id,
                "email": email,
                "phone": phone,
                "role": role,
                "avatar_url": avatar_url,
                "created_at": created_at.to_rfc3339()
            }))
        }
        None => Err("Profile not found".to_string()),
    }
}

pub async fn update_profile(pool: &PgPool, user_id: Uuid, phone: Option<String>) -> Result<serde_json::Value, String> {
    let row = sqlx::query(
        "UPDATE profiles SET phone = $2, updated_at = NOW() WHERE id = $1 RETURNING id, email, phone, role, avatar_url, created_at, updated_at"
    )
    .bind(user_id)
    .bind(phone)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    let id: Uuid = row.get("id");
    let email: String = row.get("email");
    let phone: Option<String> = row.get("phone");
    let role: String = row.get("role");
    let avatar_url: Option<String> = row.get("avatar_url");
    let created_at: chrono::DateTime<Utc> = row.get("created_at");
    let updated_at: chrono::DateTime<Utc> = row.get("updated_at");

    Ok(serde_json::json!({
        "id": id,
        "email": email,
        "phone": phone,
        "role": role,
        "avatar_url": avatar_url,
        "created_at": created_at.to_rfc3339(),
        "updated_at": updated_at.to_rfc3339()
    }))
}

pub async fn get_all_profiles(pool: &PgPool, role: Option<String>) -> Result<Vec<serde_json::Value>, String> {
    let query = match role {
        Some(ref r) => sqlx::query("SELECT id, email, phone, role, avatar_url, created_at FROM profiles WHERE role = $1"),
        None => sqlx::query("SELECT id, email, phone, role, avatar_url, created_at FROM profiles"),
    };

    let rows = if let Some(ref r) = role {
        query.bind(r).fetch_all(pool).await.map_err(|e| e.to_string())?
    } else {
        query.fetch_all(pool).await.map_err(|e| e.to_string())?
    };

    let mut profiles = Vec::new();
    for row in rows {
        let id: Uuid = row.get("id");
        let email: String = row.get("email");
        let phone: Option<String> = row.get("phone");
        let role: String = row.get("role");
        let avatar_url: Option<String> = row.get("avatar_url");
        let created_at: chrono::DateTime<Utc> = row.get("created_at");

        profiles.push(serde_json::json!({
            "id": id,
            "email": email,
            "phone": phone,
            "role": role,
            "avatar_url": avatar_url,
            "created_at": created_at.to_rfc3339()
        }));
    }

    Ok(profiles)
}
