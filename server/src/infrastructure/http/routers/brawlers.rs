use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};

use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    domain::value_objects::brawler_model::RegisterBrawlerModel,
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::brawlers::BrawlerPostgres,
        },
    },
};

pub fn routers(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = Arc::new(BrawlerPostgres::new(db_pool));
    let use_case = Arc::new(BrawlersUseCase::new(repository));

    Router::new()
        .route("/register", post(register_brawler))
        .with_state(use_case)
}

pub async fn register_brawler(
    State(user_case): State<Arc<BrawlersUseCase<BrawlerPostgres>>>,
    Json(model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse {
    match user_case.register(model).await {
        Ok(user_id) => (StatusCode::CREATED, user_id.to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
