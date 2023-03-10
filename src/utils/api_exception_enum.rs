use std::fmt::Display;

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
    pub fn error_02(msg: impl Into<String> + Display) -> ApiException {
        ApiException::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error save: {msg}") ,
            COMPONENT,
        )
    }
    pub fn error_03(msg: impl Into<String> + Display) -> ApiException {
        ApiException::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error get all provincia db: {msg}"),
            COMPONENT,
        )
    }
    pub fn error_04(msg: impl Into<String> + Display) -> ApiException {
        ApiException::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error delete all provincia db: {msg}"),
            COMPONENT,
        )
    }
    pub fn error_05() -> ApiException {
        ApiException::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error call api count comisaria",
            COMPONENT,
        )
    }
}
