mod args;
mod axumserver;
mod popdata;
mod variant;
mod webversion;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::axumserver::variant_alt;
use crate::axumserver::variant_id;
use crate::axumserver::variant_pos;
use crate::axumserver::variant_ref;
use crate::popdata::variantdatabase;
use crate::webversion::variant_alt_web;
use crate::webversion::variant_id_web;
use crate::webversion::variant_pos_web;
use crate::webversion::variant_ref_web;
use axum::Router;
use axum::routing::get;
use axum_template::engine::Engine;
use clap::Parser;
use handlebars::Handlebars;
use tower_http::services::ServeFile;

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
            threads,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = variantdatabase(variantfolder).unwrap();
                println!("The command has been finished:{:?}", command);
            });
        }
        Commands::VariantServerPostman { variantserver } => {
            if variantserver == "run" {
                println!("The server is listening on the 127.0.0.1:1000");
                let app = Router::new()
                    .route("/idsearch/{id}", get(variant_id))
                    .route("/possearch/{pos}", get(variant_pos))
                    .route("/refsearch/{ref}", get(variant_ref))
                    .route("/altsearch/{alt}", get(variant_alt));
                let listener = tokio::net::TcpListener::bind("127.0.0.1:1000")
                    .await
                    .unwrap();
                axum::serve(listener, app).await.unwrap();
            }
        }
        Commands::Webversion { variantserver } => {
            if variantserver == "run" {
                let mut hbs = Handlebars::new();
                println!("The server is listening on 127.0.0.1:4000");
                hbs.register_template_string("iddisplay", "/templates/iddisplay.hbs")
                    .expect("file not found");
                let app = Router::new()
                    .nest_service("/introduction", ServeFile::new("/templates/index.hbs"))
                    .route("/idsearch/{id}", get(variant_id_web))
                    .route("/possearch/{pos}", get(variant_pos_web))
                    .route("/altsearch{alt}", get(variant_alt_web))
                    .route("/refsearch{ref}", get(variant_ref_web))
                    .with_state(Engine::from(hbs));
                let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
                    .await
                    .unwrap();
                axum::serve(listener, app).await.unwrap();
            }
        }
    }
}
