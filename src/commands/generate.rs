use crate::toml::template::{default_variant, load_project_template, FileSpec, ProjectPost, ProjectTemplate, ProjectTool};
use log::{error, info, trace};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs, io};

pub fn generate(prefix: &str, project_name: &str, template_key: &str, variant: &str) {


    let template = load_project_template(project_name, template_key);
    check_binaries(&template);
    initialize_root(project_name, prefix, &template);
    add_dependencies(
        &template.project.tool.binary,
        &template.project.tool.commands.add_dependency,
        &template.project.dependencies,
    );
    add_dependencies(
        &template.project.tool.binary,
        &template.project.tool.commands.add_development_dependency,
        &template.project.dev_dependencies,
    );
    write_all_files(&template, variant);
    run_post_commands(&template.project.post);
    info!(
        "Successfully created your project: {}.  Happy Coding!",
        project_name
    );
}

fn initialize_root(project_name: &str, prefix: &str, config: &ProjectTemplate) {
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
    run_command(tool_binary, &args)
}

fn run_silent_command(command: &str, args: &Vec<String>) {
    Command::new(command).args(args).output().unwrap();
}

fn run_command(command: &str, args: &Vec<String>) {
    let cmd = Command::new(command).args(args).output().unwrap();
    let stdout = String::from_utf8(cmd.stdout).unwrap();
    let stderr = String::from_utf8(cmd.stderr).unwrap();
    info!("Running command \"{}\" with args {:?}\n", command, args);
    if !cmd.status.success() {
        error!("Command {} did not exit cleanly:", command);
        error!("{}", stderr);
        error!("{}", stdout);
    } else {
        trace!("{}", stdout);
    }
}

fn write_all_files(template: &ProjectTemplate, variant: &str) {
    let source_dir = PathBuf::from(&template.code.directories.source);
    let test_dir = PathBuf::from(&template.code.directories.test);

    let source_code_files = &template.code.source;
    write_files(source_code_files, source_dir, variant);

    let test_code_files = &template.code.test;
    write_files(test_code_files, test_dir, variant);

    let config_files = &template.config;
    write_files(config_files, PathBuf::from(""), variant);
}

fn write_files(files: &Vec<FileSpec>, base_prefix: PathBuf, variant: &str) {
    let file_map = collect_files_from_variants(files, variant);
    for file_spec in file_map.values() {
        let path = base_prefix.join(&file_spec.file);
        let dirs = path.parent().unwrap().to_path_buf();
        // TODO: This is unsafe.  the path should be checked prior to creation
        mkdirhier(&dirs).unwrap();
        let mut file = File::create(&path).expect("Unable to write file.");
        file.write_all(file_spec.contents.as_bytes()).unwrap();
    }
}

fn collect_files_from_variants<'a>(
    file_list: &'a Vec<FileSpec>,
    variant: &str,
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

fn check_binaries(template: &ProjectTemplate) {
    info!("CHECKING BINARIES...");
    run_silent_command(&template.language.binary, &Vec::new());
    run_silent_command(&template.project.tool.binary, &Vec::new());
    info!("CHECKING BINARIES SUCCESSFUL!");
}

fn initialize_directory(project_tool: &ProjectTool) {
    run_command(&project_tool.binary, &project_tool.commands.initialize);
}

fn run_post_commands(post: &Option<ProjectPost>) {
    if let Some(post) = post {
        for command in &post.commands {
            run_command(&command.command, &command.args)
        }
    }
}
