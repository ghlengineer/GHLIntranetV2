use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct AltSolns {
    pub id: i32,
    pub building_code: String,
    pub code_reference_sentence: String,
    pub keywords: String,
    pub ahj: String,
    pub project: String,
    pub functional_statement: String,
    pub objective: String,
    pub date_finalized: String
}