use crate::models::api_exception::ApiException;
use crate::models::departamento_response::DepartamentoResponse;
use crate::models::distrito_document::DistritoDocument;
use crate::models::provincia_document::ProvinciaDocument;
use crate::routes::init::AppState;
use crate::services::ubigeo_service::{self, UbigeoService};
use axum::extract::State;
use axum::{routing::get, Json, Router};
use dotenvy_macro::dotenv;

pub fn ubigeo_controller() -> Router<AppState> {
    Router::new()
        .route("/getalldpto", get(get_all_dpto))
        .route("/getalldptobd", get(get_all_dpto_bd))
        .route("/getallprov", get(get_all_prov))
        .route("/getalldist", get(get_all_dist))
}

async fn get_all_dpto_bd(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<DepartamentoResponse>>, ApiException> {
    let departamentos = UbigeoService::new(app_state.ubigeo_repository.clone(),
    app_state.producer_kafka.clone())
        .get_all_dpto_bd()
        .await?;
    Ok(Json(departamentos))
}
async fn get_all_dpto(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<DepartamentoResponse>>, ApiException> {
    let url_dpto = dotenv!("URL_DPTO");
    let host = dotenv!("HOST");
    let origin = dotenv!("ORIGIN");
    let departamentos = ubigeo_service::UbigeoService::new(app_state.ubigeo_repository.clone(),
    app_state.producer_kafka.clone())
        .get_all_dpto(url_dpto.to_owned(), host, origin)
        .await?;
    Ok(Json(departamentos))
}
async fn get_all_prov(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ProvinciaDocument>>, ApiException> {
    let url_prov = dotenv!("URL_PRO");
    let host = dotenv!("HOST");
    let origin = dotenv!("ORIGIN");
    let provincias = ubigeo_service::UbigeoService::new(app_state.ubigeo_repository.clone(),
    app_state.producer_kafka.clone())
        .get_add_prov(url_prov.to_owned(), host, origin)
        .await?;
    Ok(Json(provincias))
}

async fn get_all_dist(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<DistritoDocument>>, ApiException> {
    let url_dist = dotenv!("URL_DIST");
    let host = dotenv!("HOST");
    let origin = dotenv!("ORIGIN");
    let distritos = ubigeo_service::UbigeoService::new(app_state.ubigeo_repository.clone(),
    app_state.producer_kafka.clone())
        .get_add_dist(url_dist.to_owned(), host, origin)
        .await?;
    Ok(Json(distritos))
}
