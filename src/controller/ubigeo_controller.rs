use std::sync::Arc;

use crate::db::mongo_db::MongoDb;
use crate::models::api_exception::ApiException;
use crate::models::departamento_response::DepartamentoResponse;
use crate::routes::init::AppState;
use crate::services::ubigeo_service;
use axum::extract::State;
use axum::{routing::get, Json, Router};
use dotenvy_macro::dotenv;

pub fn ubigeo_controller(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/getalldpto", get(get_all_dpto))
        .with_state(app_state.clone())
}

async fn get_all_dpto(State(app_state): State<Arc<AppState>>) -> Result<Json<Vec<DepartamentoResponse>>, ApiException> {
    let url_dpto = dotenv!("URL_DPTO");
    let host = dotenv!("HOST");
    let origin = dotenv!("ORIGIN");
    let dd = Arc::clone(&app_state);
    let dddd = dd.get_db();
    ddd.init()
    let departamentos = ubigeo_service::UbigeoService::new(dddd.i)
        .get_all_dpto(url_dpto, host, origin)
        .await?;
    Ok(Json(departamentos))
}
