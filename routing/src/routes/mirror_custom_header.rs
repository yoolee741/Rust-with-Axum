use axum::http::{HeaderMap, HeaderValue};


pub async fn mirror_custom_header(headers: HeaderMap) -> String {
    let message_value: &HeaderValue = headers.get("x-message").unwrap();
    let message: String = message_value.to_str().unwrap().to_owned();

    message 

}

// HeaderMap allows us to get any of the headers that we want, not just custom one