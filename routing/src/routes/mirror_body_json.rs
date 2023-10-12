use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String
}

// it needs to serialize because when it goes out, and it's turning out from rust struct into a string
#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String
}
pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
   
    Json(MirrorJsonResponse {
        message: body.message,
        message_from_server: "Hello from Axum".to_owned()
    })

}