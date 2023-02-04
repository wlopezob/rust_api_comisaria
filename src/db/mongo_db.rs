use mongodb::{Client, options::ClientOptions};

pub struct MongoDb {
    db: DataBase
}

impl MongoDb {
    pub async fn init(connection: impl Into<String>) ->Result<Self> {
        let cliente = ClientOptions::parse(connection.into()).await?;

        todo!()
    }
}