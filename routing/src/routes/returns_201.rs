use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub async fn returns_201(el: String) -> Response {
    (StatusCode::CREATED, el.to_owned()).into_response()
}