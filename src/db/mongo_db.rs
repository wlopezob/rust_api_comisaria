use mongodb::{Client, options::ClientOptions, Database};

use crate::Result;

#[derive(Clone, Debug)]
pub struct MongoDb {
    db: Database
}

impl MongoDb {
    pub async fn init(connection: impl Into<String>, name_database: impl Into<String>) ->Result<Self> {
        let mut client_options = ClientOptions::parse(connection.into()).await?;
        client_options.app_name = Some("api_comisaria".to_owned());
        let client = Client::with_options(client_options)?;
        Ok(MongoDb{
            db: client.database(&name_database.into())
        })
    }
}