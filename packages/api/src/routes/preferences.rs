use axum::{Router, routing::{get, post, put}};

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/projects/{project_id}/preferences",
            get(get_preferences).post(set_preferences).put(update_preferences),
        )
}

async fn get_preferences() -> &'static str { "GET" }
async fn set_preferences() -> &'static str { "POST" }
async fn update_preferences() -> &'static str { "PUT" }
