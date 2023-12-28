use axum::Router;
mod user;

pub fn router() -> Router {
    let api_v1 = Router::new()
        .nest("/api/v1", user::user_router());

    Router::new()
        .nest("/", api_v1)
}
