use axum::http::HeaderValue;
use axum::{routing::get, Json, Router};
use dotenvy_macro::dotenv;
use reqwest::header::{HOST, ORIGIN};
use serde_json::Value;

use crate::models::api_exception::ApiException;
use crate::utils::api_exception_enum::ApiExceptionEnum;
use crate::{models::departamento_request::DepartamentoRequest, utils::util::remove_duplicates};

pub fn ubigeo_controller() -> Router {
    Router::new().route("/getalldpto", get(get_all_dpto))
}

async fn get_all_dpto() -> Result<Json<DepartamentoRequest>,ApiException> {
    let url_dpto = dotenv!("URL_DPTO");
    let host = dotenv!("HOST");
    let origin = dotenv!("ORIGIN");
    let client = reqwest::Client::builder().build().unwrap();
    let response = client
        .get(url_dpto)
        .header(
            HOST,
            HeaderValue::from_static(host),
        )
        .header(
            ORIGIN,
            HeaderValue::from_static(origin),
        )
        .send()
        .await.map_err(|_err| {ApiExceptionEnum::error_01()})?;

    let response_text = response.text().await.unwrap();
    let value_json: Value = serde_json::from_str(&response_text).unwrap();
    let value_json = remove_duplicates(&value_json);
    // dbg!(&response_text);
    let departamento_request: DepartamentoRequest = serde_json::from_value(value_json).unwrap();
    Ok(Json(departamento_request))
}
