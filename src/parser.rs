use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// a prefix to use for the main directory (will be prepended to the projects name)
    #[arg(short, long, default_value = "")]
    pub prefix: String,

    /// The name of the project to generate.
    #[arg()]
    pub project_name: String,

    /// The language to generate a minimal tdd project for
    #[arg()]
    pub language: String,

    /// The variant of files to select for overiding defaults
    #[arg(value_enum, short, long, default_value = "default")]
    pub variant: String,
}
