use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct VariantDatabase {
    pub filename: String,
    pub chrom: String,
    pub pos: String,
    pub id: String,
    pub refallele: String,
    pub altallele: String,
    pub quality: usize,
    pub filter: String,
}

#[derive(Debug, Serialize)]
pub struct ListTemplate {
    pub searchid: String,
    pub returnvec: Vec<VariantDatabase>,
}
