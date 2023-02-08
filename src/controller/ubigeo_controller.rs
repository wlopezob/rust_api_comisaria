use crate::models::api_exception::ApiException;
use crate::models::departamento_response::DepartamentoResponse;
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
        .route("/getalldprov", get(get_all_prov))
}

async fn get_all_dpto_bd(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<DepartamentoResponse>>, ApiException> {
    let departamentos = UbigeoService::new(app_state.ubigeo_repository.clone())
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
    let departamentos = ubigeo_service::UbigeoService::new(app_state.ubigeo_repository.clone())
        .get_all_dpto(url_dpto, host, origin)
        .await?;
    Ok(Json(departamentos))
}
async fn get_all_prov(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<ProvinciaDocument>>, ApiException> {
    let url_prov = dotenv!("URL_PRO");
    let host = dotenv!("HOST");
    let origin = dotenv!("ORIGIN");
    let provincias = ubigeo_service::UbigeoService::new(app_state.ubigeo_repository.clone())
        .get_add_prov(url_prov, host, origin)
        .await?;
    Ok(Json(provincias))
}
