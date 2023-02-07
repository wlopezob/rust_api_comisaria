use std::sync::Arc;

use mongodb::Database;

use crate::{models::departamento_response::DepartamentoResponse, utils::error::Error};

pub struct UbigeoRepository {
    db: Arc<Database>,
}

impl UbigeoRepository {
    pub fn new(db: Arc<Database>) -> Self {
        UbigeoRepository { db }
    }

    pub fn hola(&self) -> String {
        "hola".to_owned()
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
            .map_err(|error|
                {
                    return Error::MongoError(error);
                }
            )?;
        Ok(departamentos)
    }
}
