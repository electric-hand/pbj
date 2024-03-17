use clap::{Parser, ValueEnum};
use serde::Deserialize;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// a prefix to use for the main directory (will be prepended to the projects name)
    #[arg(short, default_value="")]
    pub prefix: String,

    /// The name of the project to generate.
    #[arg()]
    pub project_name: String,

    /// The language to generate a minimal tdd project for
    #[arg()]
    pub language: String,

    /// the kind of project to generate
    #[arg(value_enum, short, long, default_value = "basic")]
    pub variant: CodeVariant,
}

/// Doc comment
#[derive(ValueEnum, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CodeVariant {
    /// leet code ready project
    Leetcode,
    /// generic project (default)
    Basic,
}

impl Default for CodeVariant {
    fn default() -> Self {
        CodeVariant::Basic
    }
}
