#![allow(dead_code)]
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

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
    pub initializes_in_project_directory: bool,
    pub commands: ProjectToolCommands,
}

#[derive(Debug, Deserialize)]
pub struct ProjectToolCommands {
    pub initialize: Vec<String>,
    pub add_development_dependency: Vec<String>,
    pub add_dependency: Vec<String>,
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
    pub variant: Option<String>,
}

pub fn load_configuration(project_name: &str, language: &str) -> TestDrivenConfig {
    let path = format!("./templates/{language}.toml");
    let toml_file = fs::read_to_string(path)
        .expect("Unable to read toml in to string.")
        .replace(PROJECT_NAME_REPLACEMENT, &project_name);
    toml::from_str(&toml_file).expect("toml parsing failed.")
}
