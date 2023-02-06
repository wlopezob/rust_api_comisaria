use std::sync::Arc;

use axum::{routing::get,Router, extract::State};
use dotenvy_macro::dotenv;

use crate::{
    controller::{
        comisaria_controller::comisaria_controller, home_controller::home_controller,
        ubigeo_controller::ubigeo_controller,
    },
    db::mongo_db::{self, MongoDb},
};

#[derive(Clone)]
pub struct AppState {
    pub mongo_db: Arc<MongoDb>
}

impl AppState {
    pub fn new (db: MongoDb) -> Self {
        AppState { mongo_db: Arc::new(db) }
    }
    
    pub fn get_db(&self) -> Arc<MongoDb> {
        self.mongo_db
    }
}

pub async fn run() {
    let port = dotenv!("SERVER_PORT");
    let database_connection = dotenv!("DATABASE_CONNECTION");
    let name_database = dotenv!("NAME_DATABASE");
    let db =  mongo_db::MongoDb::init(database_connection, name_database)
        .await.expect("Error load database");
    dbg!(&port);
    let app = init_router(db);

    axum::Server::bind(&format!("0.0.0.0:{port}").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_router(db: MongoDb) -> Router {
    let app_state = Arc::new(AppState::new(db));
    Router::new().nest("/api/v1", chidren_router(app_state))
}

fn chidren_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/home", home_controller())
        .nest("/ubigeo", ubigeo_controller(app_state))
        .nest("/comisaria", comisaria_controller())
}


async fn demo(app_state: State<AppState>)  -> String {
    dbg!(&app_state.mongo_db);
    "demo".to_owned()
}