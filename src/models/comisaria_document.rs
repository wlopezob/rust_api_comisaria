use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ComisariaDocument {
    id_dpto: String,
    departamento: String,
    id_prov: String,
    provincia: String,
    id_dist: String,
    distrito: String,
    comisaria: String,
    categoria: String,
    macroregpol: String,
    regionpol: String,
    resolucion: String,
    x: String,
    y: String,
}

impl ComisariaDocument {
    pub fn new(
        id_dpto: String,
        departamento: String,
        id_prov: String,
        provincia: String,
        id_dist: String,
        distrito: String,
        comisaria: String,
        categoria: String,
        macroregpol: String,
        regionpol: String,
        resolucion: String,
        x: String,
        y: String,
    ) -> Self {
        Self {
            id_dpto,
            departamento,
            id_prov,
            provincia,
            id_dist,
            distrito,
            comisaria,
            categoria,
            macroregpol,
            regionpol,
            resolucion,
            x,
            y,
        }
    }
}
