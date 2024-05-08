#![allow(dead_code)]
use log::error;
use serde::Deserialize;
use std::{path::PathBuf, process};

use crate::{constants::{DEFAULT_VARIANT_VALUE, PROJECT_NAME_REPLACEMENT}, files::get_template};


#[derive(Debug, Deserialize)]
pub struct ProjectTemplate {
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
    pub post: Option<ProjectPost>,
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
pub struct ProjectPost {
    pub commands: Vec<Command>,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub command: String,
    pub args: Vec<String>,
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
    pub variant: String,
}

pub fn default_variant() -> String {
    DEFAULT_VARIANT_VALUE.to_string()
}

pub fn load_project_template(project_name: &str, template: &str) -> ProjectTemplate {
    if let Some(template) = get_template(template) {
        let template = template.replace(PROJECT_NAME_REPLACEMENT, &project_name);
        toml::from_str(&template).expect(format!("Failed to parse project template {}.", template).as_str())
    } else {
        error!("Failed to load project template for {}!\nPlease check that the template \"{}.toml\" exists and is in your \"templates\" directory.", template, template);
        process::exit(1);
    }
}
