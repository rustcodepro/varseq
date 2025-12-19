use crate::variant::VariantDatabase;
use sqlx::PgPool;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

#[tokio::main]
pub async fn variantdatabase(
    pathvariant: &str,
    database: &str,
) -> Result<Vec<VariantDatabase>, Box<dyn Error>> {
    let mut variantstorage: Vec<VariantDatabase> = Vec::new();
    for i in std::fs::read_dir(pathvariant)? {
        let openfile = i?.path();
        let path_str = openfile.to_str().unwrap();
        let fileopen = File::open(path_str).expect("file not found");
        let filerename = path_str.split("/").collect::<Vec<_>>()[1]
            .split(".")
            .collect::<Vec<_>>()[0]
            .to_string();
        let fileread = BufReader::new(fileopen);
        for i in fileread.lines() {
            let line = i.expect("line not present");
            let linevec = line.split("\t").collect::<Vec<_>>();
            variantstorage.push(VariantDatabase {
                filename: filerename.clone(),
                chrom: linevec[0].to_string(),
                pos: linevec[1].to_string(),
                id: linevec[2].to_string(),
                refallele: linevec[3].to_string(),
                altallele: linevec[4].to_string(),
                quality: linevec[5].parse::<usize>().unwrap(),
                filter: linevec[6].to_string(),
            })
        }
    }

    let databaseopen = PgPool::connect(database).await.unwrap();
    sqlx::query(
        r#" CREATE TABLE IF NOT EXISTS variants(
            filename integer primary key,
            chrom text not null,
            pos text not null,
            id text not null,
            refallele text not null unique,
            altallele text not null,
            quality text not null,
            filter text not null"#,
    )
    .execute(&databaseopen)
    .await?;
    for i in variantstorage.iter() {
        sqlx::query(
            "INSERT INTO variants(filename, chrom, pos, id, refallele, altlallele, quality, filter)
            values ( $1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(i.filename.clone())
        .bind(i.chrom.clone())
        .bind(i.pos.clone())
        .bind(i.id.clone())
        .bind(i.refallele.clone())
        .bind(i.altallele.clone())
        .bind(i.quality.to_string().clone())
        .bind(i.filter.clone());
    }

    Ok(variantstorage)
}
