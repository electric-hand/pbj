use clap::{Parser, Args};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {

    /// The language to generate a minimal tdd project for
    #[arg()]
    pub language: String

}