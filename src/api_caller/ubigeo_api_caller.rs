use std::time::Duration;

use crate::models::api_exception::ApiException;
use crate::models::distrito_response::DistritoResponse;
use crate::models::provincia_response::ProvinciaResponse;
use crate::utils::api_exception_enum::ApiExceptionEnum;
use crate::utils::util::get_value_response;
use crate::{models::departamento_request::DepartamentoRequest};
use axum::http::HeaderValue;
use reqwest::header::{HOST, ORIGIN};

pub struct ApiCaller {
    url: String,
}
impl ApiCaller {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
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
        let departamento_request = get_value_response::<DepartamentoRequest>(response_text);
        Ok(departamento_request)
    }

    pub async fn api_get_all_pro(
        &self,
        host: &'static str,
        origin: &'static str,
    ) -> Result<ProvinciaResponse, ApiException> {
        let client = reqwest::Client::builder().build().unwrap();
        let response_text = loop {
            let response = client
                .get(&self.url)
                .timeout(Duration::from_secs(60))
                .header(HOST, HeaderValue::from_static(host))
                .header(ORIGIN, HeaderValue::from_static(origin))
                .send()
                .await;
            dbg!(response.is_ok());
            if response.is_ok() {
                break response.unwrap().text().await.unwrap();
            }
            //sleep 2s
            //tokio::time::sleep(Duration::from_secs(2)).await;
        };
        let provincia_resonse = get_value_response::<ProvinciaResponse>(response_text);
        Ok(provincia_resonse)
    }
    pub async fn api_get_all_dist(
        &self,
        host: &'static str,
        origin: &'static str,
    ) -> Result<DistritoResponse, ApiException> {
        let client = reqwest::Client::builder().build().unwrap();
        let response_text = loop {
            let response = client
                .get(&self.url)
                .timeout(Duration::from_secs(60))
                .header(HOST, HeaderValue::from_static(host))
                .header(ORIGIN, HeaderValue::from_static(origin))
                .send()
                .await;
                dbg!(response.is_ok());
                if response.is_ok() {
                    break response.unwrap().text().await.unwrap();
                }
        };
        let distrito_resonse = get_value_response::<DistritoResponse>(response_text);
        Ok(distrito_resonse)
    }

}
