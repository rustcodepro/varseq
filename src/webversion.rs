use crate::variant::ListTemplate;
use crate::variant::VariantDatabase;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum_template::RenderHtml;
use handlebars::Handlebars;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;

type AppEngine = axum_template::engine::Engine<Handlebars<'static>>;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub async fn variant_id_web(
    State(engine): State<AppEngine>,
    Path(variantsearch): Path<String>,
) -> impl IntoResponse {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<VariantDatabase> = sqlx::query_as("SELECT filename, chrom, pos, id, refallele, altallele, quality filter FROM variants WHERE id = $1").bind(variantsearch.clone())
        .fetch_all(&connect)
        .await.unwrap();
    let mut selectedvariant: Vec<VariantDatabase> = Vec::new();
    for i in rows.iter() {
        selectedvariant.push(VariantDatabase {
            filename: i.filename.clone(),
            chrom: i.chrom.clone(),
            pos: i.pos.clone(),
            id: i.id.clone(),
            refallele: i.refallele.clone(),
            altallele: i.altallele.clone(),
            quality: i.quality.clone(),
            filter: i.filter.clone(),
        });
    }
    RenderHtml(
        "IdSearch".to_string(),
        engine,
        ListTemplate {
            searchid: variantsearch,
            returnvec: selectedvariant,
        },
    )
}

pub async fn variant_pos_web(
    State(engine): State<AppEngine>,
    Path(variantsearch): Path<String>,
) -> impl IntoResponse {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<VariantDatabase> = sqlx::query_as("SELECT filename, chrom, pos, id, refallele, altallele, quality filter FROM variants WHERE pos = $1").bind(variantsearch.clone())
        .fetch_all(&connect)
        .await.unwrap();
    let mut returnvecpos: Vec<VariantDatabase> = Vec::new();
    for i in rows.iter() {
        returnvecpos.push(VariantDatabase {
            filename: i.filename.clone(),
            chrom: i.chrom.clone(),
            pos: i.pos.clone(),
            id: i.id.clone(),
            refallele: i.refallele.clone(),
            altallele: i.altallele.clone(),
            quality: i.quality.clone(),
            filter: i.filter.clone(),
        });
    }
    RenderHtml(
        "PosSearch",
        engine,
        ListTemplate {
            searchid: variantsearch,
            returnvec: returnvecpos,
        },
    )
}

pub async fn variant_ref_web(
    State(engine): State<AppEngine>,
    Path(variantsearch): Path<String>,
) -> impl IntoResponse {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<VariantDatabase> = sqlx::query_as("SELECT filename, chrom, pos, id, refallele, altallele, quality filter FROM variants WHERE refallele = $1").bind(variantsearch.clone())
        .fetch_all(&connect)
        .await.unwrap();
    let mut returnref: Vec<VariantDatabase> = Vec::new();
    for i in rows.iter() {
        returnref.push(VariantDatabase {
            filename: i.filename.clone(),
            chrom: i.chrom.clone(),
            pos: i.pos.clone(),
            id: i.id.clone(),
            refallele: i.refallele.clone(),
            altallele: i.altallele.clone(),
            quality: i.quality.clone(),
            filter: i.filter.clone(),
        });
    }
    RenderHtml(
        "RefAllele".to_string(),
        engine,
        ListTemplate {
            searchid: variantsearch,
            returnvec: returnref,
        },
    )
}

pub async fn variant_alt_web(
    State(engine): State<AppEngine>,
    Path(variantsearch): Path<String>,
) -> impl IntoResponse {
    let connection = SqliteConnectOptions::new()
        .filename("variantsearch.db")
        .create_if_missing(true);
    let connect = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connection)
        .await
        .unwrap();
    let rows: Vec<VariantDatabase> = sqlx::query_as("SELECT filename, chrom, pos, id, refallele, altallele, quality filter FROM variants WHERE altallele = $1").bind(variantsearch.clone())
        .fetch_all(&connect)
        .await.unwrap();
    let mut returnalt: Vec<VariantDatabase> = Vec::new();
    for i in rows.iter() {
        returnalt.push(VariantDatabase {
            filename: i.filename.clone(),
            chrom: i.chrom.clone(),
            pos: i.pos.clone(),
            id: i.id.clone(),
            refallele: i.refallele.clone(),
            altallele: i.altallele.clone(),
            quality: i.quality.clone(),
            filter: i.filter.clone(),
        });
    }
    RenderHtml(
        "AltAllele".to_string(),
        engine,
        ListTemplate {
            searchid: variantsearch,
            returnvec: returnalt,
        },
    )
}
