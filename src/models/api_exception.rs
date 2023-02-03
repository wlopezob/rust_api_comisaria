use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub struct ApiException {
    code: StatusCode,
    message: String,
    component: String,
}

#[derive(Serialize)]
pub struct ApiExceptionResponseMessage {
    message: String,
    component: String,
}

impl ApiException {
    pub fn new(code: StatusCode, message: impl Into<String>, component: impl Into<String>) -> Self {
        ApiException {
            code,
            message: message.into(),
            component: component.into(),
        }
    }
}

impl IntoResponse for ApiException {
    fn into_response(self) -> axum::response::Response {
        (
            self.code,
            Json(ApiExceptionResponseMessage{
                message: self.message,
                component: self.component
            })
        ).into_response()
    }
}
