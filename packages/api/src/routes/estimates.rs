use axum::{Router, routing::{get, post, put, patch}};

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/projects/{project_id}/estimates",
            get(list_estimates).post(create_estimate),
        )
        .route(
            "/api/projects/{project_id}/estimates/{id}",
            get(get_estimate).put(update_estimate),
        )
        .route("/api/projects/{project_id}/estimates/{id}/versions", get(list_versions))
        .route("/api/estimates/{id}/request-revision", post(request_revision))
}

async fn list_estimates() -> &'static str { "GET" }
async fn create_estimate() -> &'static str { "POST" }
async fn get_estimate() -> &'static str { "GET" }
async fn update_estimate() -> &'static str { "PUT" }
async fn list_versions() -> &'static str { "GET" }
async fn request_revision() -> &'static str { "POST" }
