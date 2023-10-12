use axum::Extension;


#[derive(Clone)]
// tuple 
pub struct HeaderMessage(pub String);

pub async fn read_middleware_custom_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}