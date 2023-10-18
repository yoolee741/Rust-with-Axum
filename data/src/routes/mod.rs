mod custom_json_extractor;
mod validate_with_serde;
mod create_task;

use axum::{
    routing::post,
    Router, Extension
};

use custom_json_extractor::custom_json_extractor;
use sea_orm::DatabaseConnection;
use validate_with_serde::validate_with_serde;
use create_task::create_task;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
    .route("/validate_data", post(validate_with_serde))
    .route("/custom_json_extractor", post(custom_json_extractor))
    .route("/tasks", post(create_task))
    .layer(Extension(database))

}