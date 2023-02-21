use crate::{
    api_caller::comisaria_api_caller::ComisariaApiCaller,
    models::{api_exception::ApiException, comisaria_document::ComisariaDocument},
    routes::init::{ComisariaRepositoryState, ProducerKafkaState}, utils::api_exception_enum::ApiExceptionEnum,
};


pub struct ComisariaService {
    pub comisaria_repository: ComisariaRepositoryState,
    pub producer_kafka: ProducerKafkaState
}

#[async_trait::async_trait]
pub trait ComisariaInterface {
    async fn get_all_comisaria(
        &self,
        url_count: String,
        url_comisaria: String,
        host: &'static str,
        origin: &'static str,
    ) -> Result<Vec<ComisariaDocument>, ApiException>;

}

#[async_trait::async_trait]
impl ComisariaInterface for ComisariaService {
    async fn get_all_comisaria(
        &self,
        url_count: String,
        url_comisaria: String,
        host: &'static str,
        origin: &'static str,
    ) -> Result<Vec<ComisariaDocument>, ApiException> {
        //get total count comisaria
        let count_comisaria = ComisariaApiCaller::new(url_count)
            .get_total_comisaria(host, origin)
            .await?;
        dbg!(count_comisaria);

        let new_url_comisaria = url_comisaria.clone()
            .replace("{total}", count_comisaria.to_string().as_str());

        dbg!(&new_url_comisaria);

        let mut comisarias = Vec::new();
        //get all comisaria
        let rs = ComisariaApiCaller::new(new_url_comisaria)
            .get_all_comisaria(host, origin)
            .await?;
        if let Some(items) = rs.features {
            for item in items.into_iter() {
                let comisaria = ComisariaDocument::new(
                    item.attributes.id_dpto,
                    item.attributes.departamento,
                    item.attributes.id_prov,
                    item.attributes.provincia,
                    item.attributes.id_dist,
                    item.attributes.distrito,
                    item.attributes.comisaria,
                    item.attributes.categoria,
                    item.attributes.macroregpol,
                    item.attributes.regionpol,
                    item.attributes.resolucion,
                    item.geometry.x.to_string(),
                    item.geometry.y.to_string(),
                );
                comisarias.push(comisaria);
            }
        }
        //save all comisaria
        self.comisaria_repository
            .insert_comisaria(comisarias.clone())
            .await
            .map_err(|error| ApiExceptionEnum::error_02(error.to_string()))?;
        Ok(comisarias)
    }
}

impl ComisariaService {
    pub fn new(comisaria_repository: ComisariaRepositoryState, producer_kafka: ProducerKafkaState) -> Box<dyn ComisariaInterface  + Send + Sync> {
        Box::new(ComisariaService {
            comisaria_repository,
            producer_kafka
        })
    }
}
