use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The name of the project to generate.
    #[arg()]
    pub project_name: String,

    /// The language to generate a minimal tdd project for
    #[arg()]
    pub language: String,

    /// the kind of project to generate
    #[arg(value_enum, short, long)]
    pub variant: Option<CodeVariant>,
}

/// Doc comment
#[derive(ValueEnum, Clone, Debug)]
pub enum CodeVariant {
    /// leet code ready project
    Leetcode,
    /// generic project (default)
    Basic,
}
