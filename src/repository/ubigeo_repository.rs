use std::sync::Arc;
use tokio_stream::StreamExt;

use mongodb::{bson::doc};

use crate::{
    db::mongo_db::MongoDbConnectionManager,
    models::{
        departamento_response::DepartamentoResponse, distrito_document::DistritoDocument,
        provincia_document::ProvinciaDocument,
    },
    utils::{
        constants::{NAME_DB_DEPARTAMENTO, NAME_DB_DISTRITO, NAME_DB_PROVINCIA},
        error::Error,
    },
};

pub struct UbigeoRepository {
    //db: Arc<Database>,
    conection_manager: Arc<MongoDbConnectionManager>,
}

impl UbigeoRepository {
    // pub fn new(db: Arc<Database>) -> Self {
    //     UbigeoRepository { db }
    // }
    pub fn new(db: Arc<MongoDbConnectionManager>) -> Self {
        UbigeoRepository {
            conection_manager: db,
        }
    }

    pub async fn insert_departamento(
        &self,
        departamentos: Vec<DepartamentoResponse>,
    ) -> Result<Vec<DepartamentoResponse>, Error> {
        let collection = self
            .conection_manager
            .get_connection()
            .await
            .get_database()
            .collection::<DepartamentoResponse>(NAME_DB_DEPARTAMENTO);

        //delete all departamentos
        collection.delete_many(doc! {}, None).await?;
        //insert some departamentos into collection
        collection
            .insert_many(departamentos.clone(), None)
            .await
            .map_err(|error| Error::MongoError(error))?;
        Ok(departamentos)
    }
    pub async fn insert_provincia(
        &self,
        provincias: Vec<ProvinciaDocument>,
    ) -> Result<Vec<ProvinciaDocument>, Error> {
        let collection = self
            .conection_manager
            .get_connection()
            .await
            .get_database()
            .collection::<ProvinciaDocument>(NAME_DB_PROVINCIA);
        //delete all provincias
        collection.delete_many(doc! {}, None).await?;
        collection
            .insert_many(provincias.clone(), None)
            .await
            .map_err(|error| Error::MongoError(error))?;
        Ok(provincias)
    }

    pub async fn delete_all_distrito(&self) -> Result<bool, Error> {
        let collection = self
            .conection_manager
            .get_connection()
            .await
            .get_database()
            .collection::<DistritoDocument>(NAME_DB_DISTRITO);
        //delete all distrito
        collection.delete_many(doc! {}, None).await?;
        Ok(true)
    }

    pub async fn insert_distrito(
        &self,
        distritos: Vec<DistritoDocument>,
    ) -> Result<Vec<DistritoDocument>, Error> {
        let collection = self
            .conection_manager
            .get_connection()
            .await
            .get_database()
            .collection::<DistritoDocument>(NAME_DB_DISTRITO);
        //insert all distrito
        collection
            .insert_many(distritos.clone(), None)
            .await
            .map_err(|error| Error::MongoError(error))?;
        Ok(distritos)
    }

    pub async fn get_all_prov(&self) -> Result<Vec<ProvinciaDocument>, Error> {
        let collection = self
            .conection_manager
            .get_connection()
            .await
            .get_database()
            .collection::<ProvinciaDocument>(NAME_DB_PROVINCIA);
        let mut result = collection
            .find(None, None)
            .await
            .map_err(|error| Error::MongoError(error))?;
        let mut provincias = Vec::new();
        while let Some(provincia) = result.try_next().await? {
            provincias.push(provincia);
        }
        Ok(provincias)
    }

    pub async fn get_all_dpto(&self) -> Result<Vec<DepartamentoResponse>, Error> {
        let collection = self
            .conection_manager
            .get_connection()
            .await
            .get_database()
            .collection::<DepartamentoResponse>(NAME_DB_DEPARTAMENTO);
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
