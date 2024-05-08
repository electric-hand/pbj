use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(visible_alias="g")]
    /// Generate a project using a declarative template
    Generate {
        /// a prefix to use for the main directory (will be prepended to the projects name)
        #[arg(short, long)]
        prefix: Option<String>,

        /// The name of the project to generate.
        #[arg()]
        project_name: String,

        /// The template to use for project generation (python and typescript included in defaults)
        #[arg(short, long)]
        template: Option<String>,

        /// The variant of files to select for overiding defaults
        #[arg(value_enum, short, long)]
        variant: Option<String>,
    },

    // print a list of built in templates
    BuiltIns
}
