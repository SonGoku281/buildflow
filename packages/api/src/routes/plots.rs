use axum::{Router, routing::{get, post, put, delete}};

pub fn router() -> Router {
    Router::new()
        .route("/api/projects/{project_id}/plots", get(list_plots).post(create_plot))
        .route(
            "/api/projects/{project_id}/plots/{id}",
            get(get_plot).put(update_plot).delete(delete_plot),
        )
}

async fn list_plots() -> &'static str { "GET" }
async fn create_plot() -> &'static str { "POST" }
async fn get_plot() -> &'static str { "GET" }
async fn update_plot() -> &'static str { "PUT" }
async fn delete_plot() -> &'static str { "DELETE" }
