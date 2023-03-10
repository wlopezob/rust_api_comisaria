use crate::api_caller::ubigeo_api_caller::ApiCaller;
use crate::models::api_exception::ApiException;
use crate::models::departamento_response::DepartamentoResponse;
use crate::models::distrito_document::DistritoDocument;
use crate::models::provincia_document::ProvinciaDocument;
use crate::routes::init::{ProducerKafkaState, UbigeoRepositoryState};
use crate::utils::api_exception_enum::ApiExceptionEnum;
pub struct UbigeoService {
    pub ubigeo_repository: UbigeoRepositoryState,
    pub producer_kafka: ProducerKafkaState,
}

impl UbigeoService {
    pub fn new(
        ubigeo_repository: UbigeoRepositoryState,
        producer_kafka: ProducerKafkaState,
    ) -> Box<dyn UbigeoServiceInterface + Send + Sync> {
        Box::new(UbigeoService {
            ubigeo_repository,
            producer_kafka,
        })
    }
}

#[async_trait::async_trait]
pub trait UbigeoServiceInterface {
    async fn get_all_dpto_bd(&self) -> Result<Vec<DepartamentoResponse>, ApiException>;

    async fn get_all_dpto(
        &self,
        url_dpto: String,
        host: &'static str,
        origin: &'static str,
    ) -> Result<Vec<DepartamentoResponse>, ApiException>;

    async fn get_add_dist(
        &self,
        url_dist: String,
        host: &'static str,
        origin: &'static str,
    ) -> Result<Vec<DistritoDocument>, ApiException>;

    async fn get_add_prov(
        &self,
        url_prov: String,
        host: &'static str,
        origin: &'static str,
    ) -> Result<Vec<ProvinciaDocument>, ApiException>;
}

#[async_trait::async_trait]
impl UbigeoServiceInterface for UbigeoService {
    async fn get_all_dpto_bd(&self) -> Result<Vec<DepartamentoResponse>, ApiException> {
        let departamentos = self
            .ubigeo_repository
            .get_all_dpto()
            .await
            .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
        Ok(departamentos)
    }

    async fn get_all_dpto(
        &self,
        url_dpto: String,
        host: &'static str,
        origin: &'static str,
    ) -> Result<Vec<DepartamentoResponse>, ApiException> {
        let rs = ApiCaller::new(url_dpto)
            .api_get_all_dpto(host, origin)
            .await?;
        let mut departamentos = Vec::new();
        for item in rs.features.into_iter() {
            let departamento = DepartamentoResponse::new()
                .id_dpto(item.attributes.id_dpto)
                .departamento(item.attributes.departamento)
                .capital(item.attributes.capital)
                .builder()
                .unwrap();
            departamentos.push(departamento);
        }

        self.ubigeo_repository
            .insert_departamento(departamentos.clone())
            .await
            .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
        //convert obj to string with format json
        let txt_departamentos = serde_json::to_string(&departamentos)
            .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
        //send data to kafka
        self.producer_kafka.send("get_all_dpto", txt_departamentos.clone()).await
            .map_err(|error| ApiExceptionEnum::error_02(error.0.to_string()))?;

        Ok(departamentos)
    }

    async fn get_add_dist(
        &self,
        url_dist: String,
        host: &'static str,
        origin: &'static str,
    ) -> Result<Vec<DistritoDocument>, ApiException> {
        let mut distritos = Vec::new();
        //delete all distritos
        self.ubigeo_repository
            .delete_all_distrito()
            .await
            .map_err(|error| ApiExceptionEnum::error_04(error.to_string()))?;

        //get all provincia
        let provincias = self
            .ubigeo_repository
            .get_all_prov()
            .await
            .map_err(|error| ApiExceptionEnum::error_03(error.to_string()))?;

        for provincia in provincias {
            let mut items_save = Vec::new();
            let new_url_prov = url_dist
                .clone()
                .replace("{id_prov}", provincia.get_id_prov());
            dbg!(&new_url_prov);
            let rs = ApiCaller::new(new_url_prov)
                .api_get_all_dist(host, origin)
                .await?;
            if let Some(features) = rs.features {
                for item in features {
                    let distrito_document = DistritoDocument::new(
                        item.attributes.id_dist,
                        item.attributes.id_prov,
                        item.attributes.id_dpto,
                        item.attributes.departamento,
                        item.attributes.provincia,
                        item.attributes.distrito,
                        item.attributes.capital,
                    );
                    distritos.push(distrito_document.clone());
                    items_save.push(distrito_document);
                }
            }
            if !items_save.is_empty() {
                self.ubigeo_repository
                    .insert_distrito(items_save)
                    .await
                    .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
            }
        }
        Ok(distritos)
    }

    async fn get_add_prov(
        &self,
        url_prov: String,
        host: &'static str,
        origin: &'static str,
    ) -> Result<Vec<ProvinciaDocument>, ApiException> {
        let mut provincias = Vec::new();
        //get all dpto database
        let departamentos = self
            .ubigeo_repository
            .get_all_dpto()
            .await
            .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
        for departamento in departamentos {
            let new_url_prov = url_prov
                .clone()
                .replace("{id_dpto}", departamento.get_id_dpto().as_str());
            dbg!(&new_url_prov);
            //sleep 2s
            //tokio::time::sleep(Duration::from_secs(2)).await;
            let rs = ApiCaller::new(new_url_prov)
                .api_get_all_pro(host, origin)
                .await?;
            if let Some(features) = rs.features {
                for item in features.into_iter() {
                    let provincia = ProvinciaDocument::new(
                        item.attributes.id_dpto,
                        item.attributes.id_prov,
                        item.attributes.provincia,
                        item.attributes.capital,
                    );
                    provincias.push(provincia);
                }
            }
        }
        let provincias = self
            .ubigeo_repository
            .insert_provincia(provincias)
            .await
            .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
        Ok(provincias)
    }
}
