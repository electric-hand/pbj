#![allow(dead_code)]
use serde::Deserialize;
use std::path::PathBuf;

use crate::templates::get_template;

const PROJECT_NAME_REPLACEMENT: &str = "$PROJECT_NAME";

#[derive(Debug, Deserialize)]
pub struct TestDrivenConfig {
    pub language: Language,
    pub project: Project,
    pub code: Code,
    pub config: Vec<FileSpec>,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    pub binary: String,
    pub version: String,
    pub name: String,
    pub file_extension: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub dependencies: Vec<String>,
    pub dev_dependencies: Vec<String>,
    pub tool: ProjectTool,
}

#[derive(Debug, Deserialize)]
pub struct ProjectTool {
    pub binary: String,
    pub commands: ProjectToolCommands,
}

#[derive(Debug, Deserialize)]
pub struct ProjectToolCommands {
    pub initialize: Vec<String>,
    pub add_development_dependency: Vec<String>,
    pub add_dependency: Vec<String>,
    pub run_tests: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Code {
    pub directories: CodeDirectories,
    pub source: Vec<FileSpec>,
    pub test: Vec<FileSpec>,
}

#[derive(Debug, Deserialize)]
pub struct CodeDirectories {
    pub source: PathBuf,
    pub test: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct FileSpec {
    pub contents: String,
    pub file: PathBuf,
    #[serde(default = "default_variant")]
    pub variant: CodeVariant,
}

use String as CodeVariant;
pub fn default_variant() -> String {
    "default".to_string()
}

pub fn load_configuration(project_name: &str, language: &str) -> TestDrivenConfig {

    if let Some(template) = get_template(language) {
        let template = template.replace(PROJECT_NAME_REPLACEMENT, &project_name);
        toml::from_str(&template).expect("toml parsing failed.")
    } else {
        panic!();
    }
}
