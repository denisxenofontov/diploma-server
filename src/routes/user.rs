use axum::{Router, routing::{get, post, put, delete}};

pub fn user_router() -> Router {
    Router::new()
        .route("/users", get(get_user))
        .route("/users", post(create_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_user() -> &'static str {
    "get_user"
}

async fn create_user() -> &'static str {
    "create_user"
}

async fn update_user() -> &'static str {
    "update_user"
}

async fn delete_user() -> &'static str {
    "delete_user"
}
