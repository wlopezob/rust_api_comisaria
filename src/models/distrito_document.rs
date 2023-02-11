use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DistritoDocument {
    pub id_dist: String,
    pub id_prov: String,
    pub id_dpto: String,
    pub departamento: String,
    pub provincia: String,
    pub distrito: String,
    pub capital: String,
}

impl DistritoDocument {
    pub fn new(
        id_dist: String,
        id_prov: String,
        id_dpto: String,
        departamento: String,
        provincia: String,
        distrito: String,
        capital: String,
    ) -> Self {
        Self {
            id_dist,
            id_prov,
            id_dpto,
            departamento,
            provincia,
            distrito,
            capital,
        }
    }
}
