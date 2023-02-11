use std::sync::Arc;
use crate::{models::comisaria_document::ComisariaDocument, utils::{constants::NAME_DB_COMISARIA, error::Error}};
use mongodb::{bson::doc, Database};

pub struct ComisariaRepository {
    db: Arc<Database>,
}

impl ComisariaRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn insert_comisaria(
        &self,
        comisarias: Vec<ComisariaDocument>,
    ) -> Result<Vec<ComisariaDocument>, Error> {
        let collection = self
            .db
            .collection::<ComisariaDocument>(NAME_DB_COMISARIA);

        //delete all departamentos
        collection.delete_many(doc! {}, None).await?;
        //insert some departamentos into collection
        collection
            .insert_many(comisarias.clone(), None)
            .await
            .map_err(|error| Error::MongoError(error))?;
        Ok(comisarias)
    }
}