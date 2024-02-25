#![allow(dead_code)]
use serde::Deserialize;

type Path = String;

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
}
