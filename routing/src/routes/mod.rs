mod hello_world;
mod mirror_body_string;

use axum::{Router, body::Body, routing::post};
use hello_world::hello_world;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router<(),Body> {
    Router::new().route("/", post(hello_world))
    .route("/mirror_body_string", post(mirror_body_string))
}

