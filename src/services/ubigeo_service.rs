use crate::api_caller::ubigeo_api_caller::ApiCaller;
use crate::models::api_exception::ApiException;
use crate::models::departamento_response::DepartamentoResponse;
use crate::models::provincia_document::ProvinciaDocument;
use crate::models::provincia_response::ProvinciaResponse;
use crate::routes::init::UbigeoRepositoryState;
use crate::utils::api_exception_enum::ApiExceptionEnum;
pub struct UbigeoService {
    pub ubigeo_repository: UbigeoRepositoryState,
}

impl UbigeoService {
    pub fn new(ubigeo_repository: UbigeoRepositoryState) -> Self {
        UbigeoService { ubigeo_repository }
    }
    pub async fn get_all_dpto_bd(&self) -> Result<Vec<DepartamentoResponse>, ApiException> {
        let departamentos = self
            .ubigeo_repository
            .get_all_dpto()
            .await
            .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
        Ok(departamentos)
    }
    pub async fn get_all_dpto(
        &self,
        url_dpto: impl Into<String>,
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
        Ok(departamentos)
    }
    pub async fn get_add_prov(
        &self,
        url_prov: impl Into<String> + Clone,
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
            let new_url_prov = (url_prov.clone().into() as String)
                .replace("{id_dpto}", departamento.get_id_dpto().as_str());
            let rs = ApiCaller::new(new_url_prov)
                .api_get_all_pro(host, origin)
                .await?;

            for item in rs.features.into_iter() {
                let provincia = ProvinciaDocument::new(
                    item.attributes.id_dpto,
                    item.attributes.id_prov,
                    item.attributes.provincia,
                    item.attributes.capital,
                );
                provincias.push(provincia);
            }
        }
        Ok(provincias)
    }
}
