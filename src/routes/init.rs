use std::sync::Arc;

use axum::{extract::State, routing::get, Router};
use dotenvy_macro::dotenv;

use crate::{
    controller::{
        comisaria_controller::comisaria_controller, home_controller::home_controller,
        ubigeo_controller::ubigeo_controller,
    },
    db::mongo_db::{self},
    repository::ubigeo_repository::UbigeoRepository,
};

#[derive(Clone)]
pub struct AppState {
    pub ubigeo_repository: UbigeoRepositoryState,
}

impl AppState {
    pub fn new(ubigeo_repository: UbigeoRepositoryState) -> Self {
        AppState { ubigeo_repository }
    }
}

pub type UbigeoRepositoryState = Arc<UbigeoRepository>;

pub async fn run() {
    let port = dotenv!("SERVER_PORT");
    let database_connection = dotenv!("DATABASE_CONNECTION");
    let name_database = dotenv!("NAME_DATABASE");
    dbg!(&database_connection);
    let db = mongo_db::MongoDb::init(database_connection, name_database)
        .await
        .expect("Error load database");
    let ubigeo_repository: UbigeoRepositoryState =
        Arc::new(UbigeoRepository::new(db.get_database()));
    let app_state = AppState::new(ubigeo_repository);
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
