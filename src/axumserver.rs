use crate::popdata::variantdatabase;
use axum::extract::Path;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub async fn variant_id(Path(variantsearch): Path<String>) -> String {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    for i in databasevec.iter() {
        if i.id == variantsearch {
            println!(
                "The variants associated with these ids are: {}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
            );
        }
    }
    "The results of the fetched id are given above".to_string()
}

pub async fn variant_pos(Path(variantsearch): Path<String>) -> String {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    for i in databasevec.iter() {
        if i.pos.parse::<usize>().unwrap() == variantsearch.parse::<usize>().unwrap() {
            println!(
                "The variants associated with these ids are: {}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
            );
        }
    }
    "The results of the fetched pos are given above".to_string()
}

pub async fn variant_ref(Path(variantsearch): Path<String>) -> String {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    for i in databasevec.iter() {
        if i.refallele == variantsearch {
            println!(
                "The variants associated with these ids are: {}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
            );
        }
    }
    "The results of the fetched ref allele are given above".to_string()
}

pub async fn variant_alt(Path(variantsearch): Path<String>) -> String {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    for i in databasevec.iter() {
        if i.altallele == variantsearch {
            println!(
                "The variants associated with these ids are: {}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
            );
        }
    }
    "The results of the fetched alt allele are given above".to_string()
}
