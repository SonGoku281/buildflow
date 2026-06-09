use axum::{Router, routing::{get, post, put, delete}};

pub fn router() -> Router {
    Router::new()
        .route("/api/projects", get(list_projects).post(create_project))
        .route("/api/projects/{id}", get(get_project).put(update_project).delete(delete_project))
}

async fn list_projects() -> &'static str {
    "GET /api/projects - List authenticated user's projects"
}

async fn create_project() -> &'static str {
    "POST /api/projects - Create new project (links to plot + preferences)"
}

async fn get_project() -> &'static str {
    "GET /api/projects/:id - Get project details with related data"
}

async fn update_project() -> &'static str {
    "PUT /api/projects/:id - Update project status/description"
}

async fn delete_project() -> &'static str {
    "DELETE /api/projects/:id - Delete project"
}
