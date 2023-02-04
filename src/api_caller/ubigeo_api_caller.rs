use crate::models::api_exception::ApiException;
use crate::utils::api_exception_enum::ApiExceptionEnum;
use crate::{models::departamento_request::DepartamentoRequest, utils::util::remove_duplicates};
use axum::{http::HeaderValue};
use reqwest::header::{HOST, ORIGIN};
use serde_json::Value;

pub struct ApiCaller {
    url: String,
}
impl ApiCaller {
    pub fn new(url: impl Into<String>) -> Self {
        ApiCaller { url: url.into() }
    }
    pub async fn api_get_all_dpto(
        &self,
        host: &'static str,
        origin: &'static str,
    ) -> Result<DepartamentoRequest, ApiException> {
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(&self.url)
            .header(HOST, HeaderValue::from_static(host))
            .header(ORIGIN, HeaderValue::from_static(origin))
            .send()
            .await
            .map_err(|_err| ApiExceptionEnum::error_01())?;

        let response_text = response.text().await.unwrap();
        let value_json: Value = serde_json::from_str(&response_text).unwrap();
        let value_json = remove_duplicates(&value_json);
        let departamento_request: DepartamentoRequest = serde_json::from_value(value_json).unwrap();
        Ok(departamento_request)
    }
}