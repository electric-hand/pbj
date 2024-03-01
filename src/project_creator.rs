use crate::config::{ProjectTool, TestDrivenConfig};
use crate::parser::CodeVariant;
use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn create_project(
    project_name: &str,
    config: &TestDrivenConfig,
    variant: &Option<CodeVariant>,
) {
    check_binaries(config);
    initialize_root(project_name, &config.project.tool);
    add_dependencies(config);
    add_dev_dependencies(config);
    write_files(config, variant)
}

fn initialize_root(project_name: &str, config: &ProjectTool) {
    if config.initializes_in_project_directory {
        run_command(
            "mkdir",
            &["-v", project_name].to_vec(),
            "Failed to successfully make project directory",
        );
        env::set_current_dir(project_name).expect(&format!(
            "Unable to set {} as the current dir for further initialization",
            project_name
        ));
        initialize_directory(config);
    } else {
        initialize_directory(config);
        env::set_current_dir(project_name)
            .expect("Unable to set {} as the current dir for further initialization");
    }
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
    Command::new(command)
        .args(args)
        .arg(dependency)
        .output()
        .expect(&format!("Failed to add dependency {}", dependency));
}

fn run_command(command: &str, args: &Vec<String>, error_message: &str) {
    Command::new(command)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(error_message);
}

fn write_files(config: &TestDrivenConfig, variant: &Option<CodeVariant>) {}

fn check_binaries(config: &TestDrivenConfig) {
    Command::new(&config.language.binary)
        .output()
        .expect(&format!(
            "language binary {} not found! Check that it is excecutable from the shell this is running from.",
            &config.language.binary
        ));

    Command::new(&config.project.tool.binary)
        .output()
        .expect(&format!(
            "project tool binary {} not found! Check that it is excecutable from the shell this is running from.",
            &config.project.tool.binary
        ));
}

fn initialize_directory(config: &ProjectTool) {
    run_command(
        &config.binary,
        &config.commands.initialize,
        &format!(
            "Unable to initialize directory using tool: {} and arguments: {:?}",
            config.binary, config.commands.initialize
        ),
    );
}
