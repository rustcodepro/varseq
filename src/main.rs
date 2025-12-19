mod args;
mod axumserver;
mod popdata;
mod variant;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::axumserver::variant_alt;
use crate::axumserver::variant_id;
use crate::axumserver::variant_pos;
use crate::axumserver::variant_ref;
use crate::popdata::variantdatabase;
use crate::variant::ReturnVarAlt;
use crate::variant::ReturnVarID;
use crate::variant::ReturnVarPos;
use crate::variant::ReturnVarRef;
use axum::{Router, routing::post};
use clap::Parser;

/*
Gaurav Sablok
codeprog@icloud.com
*/
#[tokio::main]
async fn main() {
    let argparse = CommandParse::parse();
    match &argparse.command {
        Commands::PopulateVariant {
            variantfolder,
            databaseadd,
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = variantdatabase(variantfolder, databaseadd).unwrap();
                println!("The command has been finished:{:?}", command);
            });
        }
        Commands::VariantServer { variantserver } => {
            if variantserver == "run" {
                let app = Router::new()
                    .route("/idsearch", post(variant_id(ReturnVarID)))
                    .route("/possearch", post(variant_pos(ReturnVarPos)))
                    .route("/refsearch", post(variant_ref(ReturnVarRef)))
                    .route("/altsearch", post(variant_alt(ReturnVarAlt)));
                let listener = tokio::net::TcpListener::bind("127.0.0.1:1000")
                    .await
                    .unwrap();
                axum::serve(listener, app).await.unwrap();
            }
        }
    }
}
