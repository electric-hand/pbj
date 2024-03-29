use crate::template_toml::{default_variant, FileSpec, ProjectPost, ProjectTool, ProjectTemplate};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs, io};

pub fn create_project(
    project_name: &str,
    prefix: &str,
    template: &ProjectTemplate,
    variant: &String,
) {
    check_binaries(template);
    initialize_root(project_name, prefix, template);
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
    write_all_files(template, variant);
    run_post_commands(&template.project.post);
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

fn write_all_files(template: &ProjectTemplate, variant: &String) {
    let source_dir = PathBuf::from(&template.code.directories.source);
    let test_dir = PathBuf::from(&template.code.directories.test);

    let source_code_files = &template.code.source;
    write_files(source_code_files, source_dir, variant);

    let test_code_files = &template.code.test;
    write_files(test_code_files, test_dir, variant);

    let config_files = &template.config;
    write_files(config_files, PathBuf::from(""), variant);
}

fn write_files(files: &Vec<FileSpec>, base_prefix: PathBuf, variant: &String) {
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
    variant: &String,
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
    run_silent_command(&template.language.binary, &Vec::new(), &format!(
        "language binary {} not found! Check that it is excecutable from the shell this is running from.",
        &template.language.binary));

    run_silent_command(&template.project.tool.binary, &Vec::new(), &format!(
            "project tool binary {} not found! Check that it is excecutable from the shell this is running from.",
            &template.project.tool.binary
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

fn run_post_commands(post: &Option<ProjectPost>) {
    if let Some(post) = post {
        for command in &post.commands {
            let error_message = format!(
                "Unable to run command {} with args {:?}",
                &command.command, &command.args
            );
            run_command(&command.command, &command.args, &error_message)
        }
    }
}
