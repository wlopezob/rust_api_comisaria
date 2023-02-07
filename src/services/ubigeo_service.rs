use crate::api_caller::ubigeo_api_caller::ApiCaller;
use crate::models::api_exception::ApiException;
use crate::models::departamento_response::DepartamentoResponse;
use crate::routes::init::UbigeoRepositoryState;
use crate::utils::api_exception_enum::ApiExceptionEnum;
pub struct UbigeoService {
    pub ubigeo_repository: UbigeoRepositoryState,
}

impl UbigeoService {
    pub fn new(ubigeo_repository: UbigeoRepositoryState) -> Self {
        UbigeoService { ubigeo_repository }
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
        //save in db all dpto
        let h = self.ubigeo_repository.hola();
        dbg!(h);
        self.ubigeo_repository
            .insert_departamento(departamentos.clone())
            .await
            .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
        Ok(departamentos)
    }
}
