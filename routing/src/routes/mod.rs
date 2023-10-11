mod hello_world;

use axum::{Router, body::Body, routing::post};
use hello_world::hello_world;

pub fn create_routes() -> Router<(),Body> {
    Router::new().route("/", post(hello_world))
}

