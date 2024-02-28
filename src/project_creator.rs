use crate::config::{ProjectTool, TestDrivenConfig};
use std::env;
use std::io::{self, Write};
use std::process::Command;

pub fn create_project(project_name: &str, config: &TestDrivenConfig) {
    check_binaries(config);
    initialize_root(project_name, &config.project.tool);
}

fn initialize_root(project_name: &str, config: &ProjectTool) {
    if config.initializes_in_project_directory {
        Command::new("mkdir")
            .arg(project_name)
            .output()
            .expect("Failed to successfully make project directory");
        env::set_current_dir(project_name)
            .expect("Unable to set {} as the current dir for further initialization");
        initialize(config);
    } else {
        initialize(config);
        env::set_current_dir(project_name)
            .expect("Unable to set {} as the current dir for further initialization");
    }
}

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

fn initialize(config: &ProjectTool) {
    Command::new(&config.binary)
        .args(&config.commands.initialize)
        .output()
        .expect(&format!(
            "Unable to initialize directory using tool: {} and arguments: {:?}",
            config.binary, config.commands.initialize
        ));
}
