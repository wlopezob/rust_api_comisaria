use std::time::Duration;

use crate::models::api_exception::ApiException;
use crate::models::provincia_response::ProvinciaResponse;
use crate::utils::api_exception_enum::ApiExceptionEnum;
use crate::{models::departamento_request::DepartamentoRequest, utils::util::remove_duplicates};
use axum::http::HeaderValue;
use reqwest::header::{HOST, ORIGIN};
use serde::de::DeserializeOwned;
use serde_json::Value;

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
        let departamento_request = self.get_value_response::<DepartamentoRequest>(response_text);
        Ok(departamento_request)
    }

    pub async fn api_get_all_pro(
        &self,
        host: &'static str,
        origin: &'static str,
    ) -> Result<ProvinciaResponse, ApiException> {
        let client = reqwest::Client::builder().build().unwrap();
        let mut response_text = String::from("");
        loop {
            let response = client
                .get(&self.url)
                .timeout(Duration::from_secs(60))
                .header(HOST, HeaderValue::from_static(host))
                .header(ORIGIN, HeaderValue::from_static(origin))
                .send()
                .await;
            dbg!(response.is_ok());
            if response.is_ok() {
                response_text = response.unwrap().text().await.unwrap();
                break;
            }
            //sleep 2s
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
        let provincia_resonse = self.get_value_response::<ProvinciaResponse>(response_text);
        Ok(provincia_resonse)
    }

    fn get_value_response<T: DeserializeOwned>(
        &self,
        response_text: impl Into<String> + Clone,
    ) -> T {
        //dbg!(&response_text.clone().into());
        let value_json: Value = serde_json::from_str(&response_text.into()).unwrap();
        let value_json = remove_duplicates(&value_json);
        serde_json::from_value::<T>(value_json).unwrap()
    }
}
