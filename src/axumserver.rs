use crate::variant::VariantDatabase;
use axum::extract::Path;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs::File;
use std::io::Write;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub async fn variant_id(Path(variantsearch): Path<String>) -> String {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<VariantDatabase> = sqlx::query_as("SELECT filename, chrom, pos, id, refallele, altallele, quality filter FROM variants WHERE id = $1").bind(variantsearch)
        .fetch_all(&connect)
        .await.unwrap();
    let mut filewrite = File::create("fileidvariant.txt").expect("file not present");
    for i in rows.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
        )
        .expect("file not present");
    }

    "The results of the fetched id are given above".to_string()
}

pub async fn variant_pos(Path(variantsearch): Path<String>) -> String {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<VariantDatabase> = sqlx::query_as("SELECT filename, chrom, pos, id, refallele, altallele, quality filter FROM variants WHERE pos = $1").bind(variantsearch)
        .fetch_all(&connect)
        .await.unwrap();
    let mut filewrite = File::create("fileidvariant.txt").expect("file not present");
    for i in rows.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
        )
        .expect("file not present");
    }
    "The results of the fetched pos are given above".to_string()
}

pub async fn variant_ref(Path(variantsearch): Path<String>) -> String {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<VariantDatabase> = sqlx::query_as("SELECT filename, chrom, pos, id, refallele, altallele, quality filter FROM variants WHERE refallele = $1").bind(variantsearch)
        .fetch_all(&connect)
        .await.unwrap();
    let mut filewrite = File::create("fileidvariant.txt").expect("file not present");
    for i in rows.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
        )
        .expect("file not present");
    }
    "The results of the fetched ref allele are given above".to_string()
}

pub async fn variant_alt(Path(variantsearch): Path<String>) -> String {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<VariantDatabase> = sqlx::query_as("SELECT filename, chrom, pos, id, refallele, altallele, quality filter FROM variants WHERE altallele = $1").bind(variantsearch)
        .fetch_all(&connect)
        .await.unwrap();
    let mut filewrite = File::create("fileidvariant.txt").expect("file not present");
    for i in rows.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            i.filename, i.pos, i.chrom, i.id, i.refallele, i.altallele, i.quality, i.filter
        )
        .expect("file not present");
    }
    "The results of the fetched alt allele are given above".to_string()
}
