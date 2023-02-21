use crate::{
    db::mongo_db::MongoDbConnectionManager,
    models::comisaria_document::ComisariaDocument,
    utils::{constants::NAME_DB_COMISARIA, error::Error},
};
use mongodb::{bson::doc};
use std::sync::Arc;

pub struct ComisariaRepository {
    //db: Arc<Database>,
    conection_manager: Arc<MongoDbConnectionManager>,
}

impl ComisariaRepository {
    pub fn new(db: Arc<MongoDbConnectionManager>) -> Self {
        Self {
            conection_manager: db,
        }
    }

    pub async fn insert_comisaria(
        &self,
        comisarias: Vec<ComisariaDocument>,
    ) -> Result<Vec<ComisariaDocument>, Error> {
        let collection = self
            .conection_manager
            .get_connection()
            .await
            .get_database()
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
