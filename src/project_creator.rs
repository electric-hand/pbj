use crate::config::{ProjectTool, TestDrivenConfig};
use crate::parser::CodeVariant;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs, io};

pub fn create_project(
    project_name: &str,
    config: &TestDrivenConfig,
    variant: &Option<CodeVariant>,
) {
    check_binaries(config);
    initialize_root(project_name, &config);
    add_dependencies(config);
    add_dev_dependencies(config);
    write_files(config, variant)
}

fn initialize_root(project_name: &str, config: &TestDrivenConfig) {
    let project_name = PathBuf::from(project_name);
    if config.project.tool.initializes_in_project_directory {
        mkdirhier(&project_name).expect("Could not create project directory");
        env::set_current_dir(&project_name).expect(&format!(
            "Unable to set {:?} as the current dir for further initialization",
            &project_name
        ));
        initialize_directory(&config.project.tool);
    } else {
        initialize_directory(&config.project.tool);
        env::set_current_dir(project_name)
            .expect("Unable to set {} as the current dir for further initialization");
    }

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

fn add_dependencies(config: &TestDrivenConfig) {
    for dep in &config.project.dependencies {
        add_dependency(
            &config.project.tool.binary,
            &config.project.tool.commands.add_dependency,
            dep,
        )
    }
}

fn add_dev_dependencies(config: &TestDrivenConfig) {
    for dep in &config.project.dev_dependencies {
        add_dependency(
            &config.project.tool.binary,
            &config.project.tool.commands.add_development_dependency,
            dep,
        )
    }
}

fn add_dependency(command: &str, args: &Vec<String>, dependency: &str) {
    let mut args: Vec<String> = args.clone();
    args.push(dependency.to_string());
    run_command(
        command,
        &args,
        &format!("Failed to add dependency {}", dependency),
    );
}

fn run_command(command: &str, args: &Vec<String>, error_message: &str) {
    Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(error_message);
}

fn write_files(config: &TestDrivenConfig, variant: &Option<CodeVariant>) {
    let source_dir = PathBuf::from(&config.code.directories.source);
    let test_dir = PathBuf::from(&config.code.directories.test);

    let source_code_files = &config.code.source;
    let test_code_files = &config.code.test;
    let config_files = &config.config;

    for file_spec in source_code_files {
        let path = source_dir.join(&file_spec.file);
        let mut file = File::create(&path).expect("couldn't create file");
        file.write_all(file_spec.contents.as_bytes())
            .expect("Couldn't write the file contents");
    }

    for file_spec in test_code_files {
        let path = test_dir.join(&file_spec.file);
        let mut file = File::create(&path).expect("couldn't create file");
        file.write_all(file_spec.contents.as_bytes())
            .expect("Couldn't write the file contents");
    }

    for file_spec in config_files {
        let path = PathBuf::from(&file_spec.file);
        let dirs = path.parent().expect("gimme the parent").to_path_buf();
        mkdirhier(&dirs).expect("message");
        let mut file = File::create(&path).expect("couldn't create file");
        file.write_all(file_spec.contents.as_bytes())
            .expect("Couldn't write the file contents");
    }


}

fn check_binaries(config: &TestDrivenConfig) {
    run_command(&config.language.binary, &Vec::new(), &format!(
        "language binary {} not found! Check that it is excecutable from the shell this is running from.",
        &config.language.binary));

    run_command(&config.project.tool.binary, &Vec::new(), &format!(
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
