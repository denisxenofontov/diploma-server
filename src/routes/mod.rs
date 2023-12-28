use axum::Router;
use sqlx::{Pool, Postgres};
mod user;

pub fn router(pool: Pool<Postgres>) -> Router {
    let api_v1 = Router::new()
        .nest("/api/v1", user::user_router());

    Router::new()
        .nest("/", api_v1)
}
