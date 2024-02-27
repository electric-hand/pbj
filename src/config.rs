#![allow(dead_code)]
use std::fs;

use serde::Deserialize;

type Path = String;
const PROJECT_NAME_REPLACEMENT: &str="$PROJECT_NAME";

#[derive(Debug, Deserialize)]
pub struct TestDrivenConfig {
    language: Language,
    project: Project,
    code: Code,
    config: Vec<FileSpec>,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    binary: String,
    version: String,
    name: String,
    file_extension: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    dependencies: Vec<String>,
    dev_dependencies: Vec<String>,
    tool: ProjectTool,
}

#[derive(Debug, Deserialize)]
pub struct ProjectTool {
    binary: String,
    makes_base_directory: bool,
    commands: ProjectToolCommands,
}

#[derive(Debug, Deserialize)]
pub struct ProjectToolCommands {
    initialize: Vec<String>,
    add_development_dependency: Vec<String>,
    add_dependency: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Code {
    directories: CodeDirectories,
    source: Vec<FileSpec>,
    test: Vec<FileSpec>,
}

#[derive(Debug, Deserialize)]
pub struct CodeDirectories {
    source: Path,
    test: Path,
}

#[derive(Debug, Deserialize)]
pub struct FileSpec {
    contents: String,
    file: Path,
    variant: Option<String>
}

pub fn load_configuration(project_name: String, language: String) -> TestDrivenConfig {
    let path = format!("./templates/{language}.toml");
    println!("{}", path);
    let toml_file = fs::read_to_string(path).expect("this to work");
    let mut config: TestDrivenConfig = toml::from_str(&toml_file).expect("bad toml");
    let test_files = &mut config.code.test;
    for file in test_files {
        let new_file_contents = file.contents.replace(PROJECT_NAME_REPLACEMENT, &project_name);
        file.contents = new_file_contents;
    }
    config
}