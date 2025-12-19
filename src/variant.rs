#[derive(Debug, Clone, PartialOrd, PartialEq)]
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

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize)]
pub struct ReturnVarID {
    pub id: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize)]
pub struct ReturnVarPos {
    pub pos: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize)]
pub struct ReturnVarRef {
    pub refallele: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize)]
pub struct ReturnVarAlt {
    pub altallele: String,
}
