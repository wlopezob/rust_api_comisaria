use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComisariaCountRequest {
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComisariaResponse {
    pub features: Option<Vec<Feature>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub attributes: Attributes,
    pub geometry: Geometry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub objectid: i64,
    #[serde(rename = "id_dpto")]
    pub id_dpto: String,
    pub departamento: String,
    #[serde(rename = "id_prov")]
    pub id_prov: String,
    pub provincia: String,
    #[serde(rename = "id_dist")]
    pub id_dist: String,
    pub distrito: String,
    pub comisaria: String,
    #[serde(rename = "tipo_comi")]
    pub tipo_comi: String,
    #[serde(rename = "clas_muni")]
    pub clas_muni: String,
    #[serde(rename = "cod_inei")]
    pub cod_inei: String,
    #[serde(rename = "cod_cpnp")]
    pub cod_cpnp: String,
    #[serde(rename = "cod_uni")]
    pub cod_uni: String,
    pub categoria: String,
    #[serde(rename = "cod_macroregpol")]
    pub cod_macroregpol: String,
    pub macroregpol: String,
    #[serde(rename = "cod_regpol")]
    pub cod_regpol: String,
    pub regionpol: String,
    #[serde(rename = "cod_divpol_divopus")]
    pub cod_divpol_divopus: String,
    #[serde(rename = "divpol_divopus")]
    pub divpol_divopus: String,
    pub resolucion: String,
    #[serde(rename = "zona_utm")]
    pub zona_utm: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    pub x: f64,
    pub y: f64,
}

