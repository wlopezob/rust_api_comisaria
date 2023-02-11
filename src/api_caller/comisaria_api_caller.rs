use axum::http::HeaderValue;
use reqwest::header::{HOST, ORIGIN};

use crate::{
    models::{
        api_exception::ApiException,
        comisaria_response::{ComisariaCountRequest, ComisariaResponse},
    },
    utils::{api_exception_enum::ApiExceptionEnum, util::get_value_response},
};

pub struct ComisariaApiCaller {
    url: String,
}

impl ComisariaApiCaller {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
    pub async fn get_total_comisaria(
        &self,
        host: &'static str,
        origin: &'static str,
    ) -> Result<i64, ApiException> {
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(&self.url)
            .header(HOST, HeaderValue::from_static(host))
            .header(ORIGIN, HeaderValue::from_static(origin))
            .send()
            .await
            .map_err(|_err| ApiExceptionEnum::error_05())?;
        let comisaria_count_request: ComisariaCountRequest = response.json().await.unwrap();
        Ok(comisaria_count_request.count)
    }

    pub async fn get_all_comisaria(
        &self,
        host: &'static str,
        origin: &'static str,
    ) -> Result<ComisariaResponse, ApiException> {
        let client = reqwest::Client::builder().build().unwrap();
        let response = client
            .get(&self.url)
            .header(HOST, HeaderValue::from_static(host))
            .header(ORIGIN, HeaderValue::from_static(origin))
            .send()
            .await
            .map_err(|_err| ApiExceptionEnum::error_01())?;

        let response_text = response.text().await.unwrap();
        let comisaria_response = get_value_response::<ComisariaResponse>(response_text);
        Ok(comisaria_response)
    }
}
