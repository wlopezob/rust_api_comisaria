use mongodb::{options::ClientOptions, Client, Database};
use tokio::sync::{Mutex, MutexGuard};

use crate::Result;

#[derive(Clone)]
pub struct MongoDb {
    db: Database,
}

impl MongoDb {
    pub async fn init(
        connection: impl Into<String>,
        name_database: impl Into<String>,
    ) -> Result<Self> {
        let mut client_options = ClientOptions::parse(connection.into()).await?;
        client_options.app_name = Some("api_comisaria".to_owned());
        let client = Client::with_options(client_options)?;
        Ok(Self {
            db: client.database(&name_database.into()),
        })
    }
    // pub fn get_database(&self) -> Arc<Database>{
    //     self.db.clone()
    // }
    pub fn get_database(&self) -> &Database {
        &self.db
    }
}

pub struct MongoDbConnectionManager {
    connection: Mutex<MongoDb>,
}

impl MongoDbConnectionManager {
    pub async fn new(
        connection: impl Into<String>,
        name_database: impl Into<String>,
    ) -> Result<Self> {
        let connection = Mutex::new(MongoDb::init(connection, name_database).await?);
        Ok(Self { connection })
    }
    pub async fn get_connection(&self) -> MutexGuard<MongoDb> {
        self.connection.lock().await
    }
}
