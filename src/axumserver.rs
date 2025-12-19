use crate::popdata::variantdatabase;
use crate::variant::ReturnVarAlt;
use crate::variant::ReturnVarID;
use crate::variant::ReturnVarPos;
use crate::variant::ReturnVarRef;
use axum::Form;
use serde::Deserialize;

pub async fn variant_id(Form(variantsearch): Form<ReturnVarID>) {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    for i in databasevec.iter() {
        if i.id == variantsearch.id {
            println!(
                "The variants associated with these ids are: {}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
            );
        }
    }
}

pub async fn variant_pos(Form(variantsearch): Form<ReturnVarPos>) {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    for i in databasevec.iter() {
        if i.pos.parse::<usize>().unwrap() == variantsearch.pos.parse::<usize>().unwrap() {
            println!(
                "The variants associated with these ids are: {}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
            );
        }
    }
}

pub async fn variant_ref(Form(variantsearch): Form<ReturnVarRef>) {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    for i in databasevec.iter() {
        if i.refallele == variantsearch.refallele {
            println!(
                "The variants associated with these ids are: {}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
            );
        }
    }
}

pub async fn variant_alt(Form(variantsearch): Form<ReturnVarAlt>) {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    for i in databasevec.iter() {
        if i.altallele == variantsearch.altallele {
            println!(
                "The variants associated with these ids are: {}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
            );
        }
    }
}
