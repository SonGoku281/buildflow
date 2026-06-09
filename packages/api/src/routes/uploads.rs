use axum::{Router, routing::{get, post, delete}};

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/projects/{project_id}/uploads",
            get(list_uploads).post(upload_file),
        )
        .route("/api/uploads/{id}", delete(delete_upload))
}

async fn list_uploads() -> &'static str { "GET" }
async fn upload_file() -> &'static str { "POST - Returns Supabase presigned URL" }
async fn delete_upload() -> &'static str { "DELETE" }
