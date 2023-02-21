use std::sync::Arc;

use axum::Router;
use dotenvy_macro::dotenv;

use crate::{
    controller::{
        comisaria_controller::comisaria_controller, home_controller::home_controller,
        ubigeo_controller::ubigeo_controller,
    },
    db::mongo_db::{MongoDbConnectionManager},
    repository::{comisaria_repository::ComisariaRepository, ubigeo_repository::UbigeoRepository}, producer::producer::ProducerKafka,
};

#[derive(Clone)]
pub struct AppState {
    pub ubigeo_repository: UbigeoRepositoryState,
    pub comisaria_repository: ComisariaRepositoryState,
    pub producer_kafka: ProducerKafkaState
}

impl AppState {
    pub fn new(
        ubigeo_repository: UbigeoRepositoryState,
        comisaria_repository: ComisariaRepositoryState,
        producer_kafka: ProducerKafkaState
    ) -> Self {
        AppState {
            ubigeo_repository,
            comisaria_repository,
            producer_kafka
        }
    }
}

pub type UbigeoRepositoryState = Arc<UbigeoRepository>;
pub type ComisariaRepositoryState = Arc<ComisariaRepository>;
pub type ProducerKafkaState = Arc<ProducerKafka>;

pub async fn run() {
    let port = dotenv!("SERVER_PORT");
    let database_connection = dotenv!("DATABASE_CONNECTION");
    let name_database = dotenv!("NAME_DATABASE");
    dbg!(&database_connection);

    let connection_manager = Arc::new(
        MongoDbConnectionManager::new(database_connection, name_database)
            .await
            .expect("Error load database"),
    );
    let ubigeo_repository: UbigeoRepositoryState =
        Arc::new(UbigeoRepository::new(connection_manager.clone()));
    let comisaria_repository: ComisariaRepositoryState =
        Arc::new(ComisariaRepository::new(connection_manager.clone()));

    //init producer kafka
    let kafka_servers = dotenv!("KAFKA_SERVER");
    let kafka_username = dotenv!("KAFKA_USERNAME");
    let kafka_password = dotenv!("KAFKA_PASSWORD");
    let kafka_topic = dotenv!("KAFKA_TOPIC");
    let producer = ProducerKafka::new(kafka_servers, kafka_username, kafka_password, kafka_topic);
    let producer_kafka: ProducerKafkaState = Arc::new(producer);

    let app_state = AppState::new(ubigeo_repository, comisaria_repository, producer_kafka);
    let app = init_router(app_state);

    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_router(app_state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", chidren_router())
        .with_state(app_state)
}

fn chidren_router() -> Router<AppState> {
    Router::new()
        .nest("/home", home_controller())
        .nest("/ubigeo", ubigeo_controller())
        .nest("/comisaria", comisaria_controller())
}
