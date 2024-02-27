use clap::Parser;

use crate::config::load_configuration;
use crate::project_creator::create_project;
use crate::parser::Cli;

pub mod config;
pub mod parser;
pub mod project_creator;


fn main() {

    let args = Cli::parse();
    let project = args.project_name;
    let language = args.language;
    let config = load_configuration(project, language);
    create_project(config);
}
