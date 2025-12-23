use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct VariantDatabase {
    pub filename: String,
    pub chrom: String,
    pub pos: String,
    pub id: String,
    pub refallele: String,
    pub altallele: String,
    pub quality: String,
    pub filter: String,
}

#[derive(Debug, Serialize)]
pub struct ListTemplate {
    pub searchid: String,
    pub returnvec: Vec<VariantDatabase>,
}
