use std::sync::Arc;
use tokio_stream::{StreamExt};

use mongodb::Database;

use crate::{models::departamento_response::DepartamentoResponse, utils::error::Error};

pub struct UbigeoRepository {
    db: Arc<Database>,
}

impl UbigeoRepository {
    pub fn new(db: Arc<Database>) -> Self {
        UbigeoRepository { db }
    }
    pub async fn insert_departamento(
        &self,
        departamentos: Vec<DepartamentoResponse>,
    ) -> Result<Vec<DepartamentoResponse>, Error> {
        let collection = self.db.collection::<DepartamentoResponse>("departamento");
        //insert some departamentos into collection
        collection
            .insert_many(departamentos.clone(), None)
            .await
            .map_err(|error| Error::MongoError(error))?;
        Ok(departamentos)
    }
    pub async fn get_all_dpto(&self) -> Result<Vec<DepartamentoResponse>, Error> {
        let collection = self.db.collection::<DepartamentoResponse>("departamento");
        let mut result = collection
            .find(None, None)
            .await
            .map_err(|error| Error::MongoError(error))?;
        let mut departamentos = Vec::new();
        while let Some(dep) = result.try_next().await? {
            departamentos.push(dep);
        }
        Ok(departamentos)
    }
}
