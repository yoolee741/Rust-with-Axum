use axum::Json;
use serde::Serialize;


#[derive(Serialize)]

pub struct GetJson {
    message: String,
    count: i32,
    username: String
}
pub async fn get_json() -> Json<GetJson> {
    let data = GetJson { message: "This is get_json message!".to_owned(), count: 7, username: "random_name".to_owned() };
    Json(data)
}