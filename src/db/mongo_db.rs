use std::sync::Arc;

use mongodb::{Client, options::ClientOptions, Database};

use crate::Result;

#[derive(Clone)]
pub struct MongoDb {
    db: Arc<Database>
}

impl MongoDb {
    pub async fn init(connection: impl Into<String>, name_database: impl Into<String>) ->Result<Self> {
        let mut client_options = ClientOptions::parse(connection.into()).await?;
        client_options.app_name = Some("api_comisaria".to_owned());
        let client = Client::with_options(client_options)?;
        Ok(Self{
            db: Arc::new(client.database(&name_database.into()))
        })
    }
    pub fn get_database(&self) -> Arc<Database>{
        self.db.clone()
    }

}