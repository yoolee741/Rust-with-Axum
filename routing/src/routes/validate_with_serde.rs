use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser{
    // if you want username optional, use option tag
    username: Option<String>,
    password: String
}

pub async fn validate_with_serde (Json(user): Json<RequestUser>) {
    dbg!(user);
}