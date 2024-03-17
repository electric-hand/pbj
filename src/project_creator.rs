use crate::config::{FileSpec, ProjectTool, TestDrivenConfig, default_variant};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs, io};

use String as CodeVariant;

pub fn create_project(
    project_name: &str,
    prefix: &str,
    config: &TestDrivenConfig,
    variant: &CodeVariant,
) {
    check_binaries(config);
    initialize_root(project_name, prefix, config);
    add_dependencies(
        &config.project.tool.binary,
        &config.project.tool.commands.add_dependency,
        &config.project.dependencies,
    );
    add_dependencies(
        &config.project.tool.binary,
        &config.project.tool.commands.add_development_dependency,
        &config.project.dev_dependencies,
    );
    write_all_files(config, variant)
}

fn initialize_root(project_name: &str, prefix: &str, config: &TestDrivenConfig) {
    let project_directory = PathBuf::from(vec![prefix, project_name].concat());

    mkdirhier(&project_directory).expect("Could not create project directory");
    env::set_current_dir(&project_directory).expect(&format!(
        "Unable to set {:?} as the current dir for further initialization",
        &project_directory
    ));

    initialize_directory(&config.project.tool);
    mkdirhier(&config.code.directories.source).expect("Could not create source directory");
    mkdirhier(&config.code.directories.test).expect("Could not create test directory")
}

fn mkdirhier(path: &PathBuf) -> io::Result<()> {
    if let Err(err) = fs::create_dir_all(path) {
        if err.kind() != io::ErrorKind::AlreadyExists {
            return Err(err);
        }
    }
    Ok(())
}

fn add_dependencies(tool_binary: &String, add_commands: &Vec<String>, packages: &Vec<String>) {
    if packages.len() == 0 {
        return;
    }
    let mut args = add_commands.clone();
    args.append(packages.clone().as_mut());
    run_command(tool_binary, &args, "")
}

fn run_silent_command(command: &str, args: &Vec<String>, error_message: &str) {
    Command::new(command)
        .args(args)
        .output()
        .expect(error_message);
}

fn run_command(command: &str, args: &Vec<String>, error_message: &str) {
    Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(error_message);
}

fn write_all_files(config: &TestDrivenConfig, variant: &CodeVariant) {
    let source_dir = PathBuf::from(&config.code.directories.source);
    let test_dir = PathBuf::from(&config.code.directories.test);

    let source_code_files = &config.code.source;
    write_files(source_code_files, source_dir, variant);

    let test_code_files = &config.code.test;
    write_files(test_code_files, test_dir, variant);

    let config_files = &config.config;
    write_files(config_files, PathBuf::from(""), variant);
}

fn write_files(files: &Vec<FileSpec>, base_prefix: PathBuf, variant: &CodeVariant) {
    let file_map = collect_files_from_variants(files, variant);
    for file_spec in file_map.values() {
        let path = base_prefix.join(&file_spec.file);
        let dirs = path.parent().expect("gimme the parent").to_path_buf();
        // TODO: This is unsafe.  the path should be checked prior to creation
        mkdirhier(&dirs).expect("message");
        let mut file = File::create(&path).expect("couldn't create file");
        file.write_all(file_spec.contents.as_bytes())
            .expect("Couldn't write the file contents");
    }
}

fn collect_files_from_variants<'a>(
    file_list: &'a Vec<FileSpec>,
    variant: &CodeVariant,
) -> HashMap<&'a PathBuf, &'a FileSpec> {
    let mut source_files: HashMap<&PathBuf, &FileSpec> = HashMap::new();

    for file in file_list {
        if &file.variant == variant {
            source_files.insert(&file.file, file);
        }
        if &file.variant == &default_variant() && !source_files.contains_key(&file.file) {
            source_files.insert(&file.file, file);
        }
    }
    return source_files;
}

fn check_binaries(config: &TestDrivenConfig) {
    run_silent_command(&config.language.binary, &Vec::new(), &format!(
        "language binary {} not found! Check that it is excecutable from the shell this is running from.",
        &config.language.binary));

    run_silent_command(&config.project.tool.binary, &Vec::new(), &format!(
            "project tool binary {} not found! Check that it is excecutable from the shell this is running from.",
            &config.project.tool.binary
        ));
}

fn initialize_directory(project_tool: &ProjectTool) {
    run_command(
        &project_tool.binary,
        &project_tool.commands.initialize,
        &format!(
            "Unable to initialize directory using tool: {} and arguments: {:?}",
            project_tool.binary, project_tool.commands.initialize
        ),
    );
}
