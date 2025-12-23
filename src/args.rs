use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "varseq",
    version = "1.0",
    about = "variant population database and web
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build the population database
    PopulateVariant {
        /// variant folder file
        variantfolder: String,
        /// number of the threads
        threads: String,
    },
    /// Axum server
    VariantServerPostman {
        /// variant server start
        variantserver: String,
    },
    /// Axum server web
    Webversion {
        /// variant server start
        variantserver: String,
    },
}
