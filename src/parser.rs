use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// a prefix to use for the main directory (will be prepended to the projects name)
    #[arg(short, long)]
    pub prefix: Option<String>,

    /// The name of the project to generate.
    #[arg()]
    pub project_name: String,

    /// The template to use for project generation (python and typescript included in defaults)
    #[arg(short, long)]
    pub template: Option<String>,

    /// The variant of files to select for overiding defaults
    #[arg(value_enum, short, long)]
    pub variant: Option<String>,
}
