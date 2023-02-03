use crate::models::api_exception::ApiException;
use axum::http::StatusCode;

const COMPONENT: &str = "api_comisaria";

pub struct ApiExceptionEnum {}

impl ApiExceptionEnum {
    pub fn error_01() -> ApiException {
        ApiException::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error call api search departamento",
            COMPONENT,
        )
    }
}
