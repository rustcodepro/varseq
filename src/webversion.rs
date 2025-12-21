use crate::popdata::variantdatabase;
use crate::variant::ListTemplate;
use crate::variant::VariantDatabase;
use axum::extract::{Path, State};
use axum_template::{Engine, RenderHtml};
use handlebars::Handlebars;

type AppEngine = Engine<Handlebars<'static>>;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub async fn variant_id_web(
    State(engine): State<AppEngine>,
    Path(variantsearch): Path<String>,
) -> RenderHtml<String, ListTemplate> {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    let mut selectedvariant: Vec<VariantDatabase> = Vec::new();
    for i in databasevec.iter() {
        if i.id == variantsearch {
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
    }
    RenderHtml(
        "IdSearch",
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
) -> RenderHtml<String, ListTemplate> {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    let mut returnvecpos: Vec<VariantDatabase> = Vec::new();
    for i in databasevec.iter() {
        if i.pos.parse::<usize>().unwrap() == variantsearch.parse::<usize>().unwrap() {
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
    }
    RenderHtml(
        "PosSearch",
        engine,
        ListTemplate {
            searchid: variantsearch,
            returnvec: returnvecpos,
        },
    );
}

pub async fn variant_ref_web(
    State(engine): State<AppEngine>,
    Path(variantsearch): Path<String>,
) -> RenderHtml<String, ListTemplate> {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    let mut returnref: Vec<VariantDatabase> = Vec::new();
    for i in databasevec.iter() {
        if i.refallele == variantsearch {
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
    }
    RenderHtml(
        "RefAllele",
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
) -> RenderHtml<String, ListTemplate> {
    let pathvariant = "";
    let database = "";
    let databasevec = variantdatabase(pathvariant, database).unwrap();
    let mut returnalt: Vec<VariantDatabase> = Vec::new();
    for i in databasevec.iter() {
        if i.altallele == variantsearch {
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
    }
    RenderHtml(
        "AltAllele",
        engine,
        ListTemplate {
            searchid: variantsearch,
            returnvec: returnalt,
        },
    )
}
