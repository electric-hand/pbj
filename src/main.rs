use clap::Parser;
use tddme::config::load_configuration;
use tddme::parser::Cli;
use tddme::project_creator::create_project;

fn main() {
    let args = Cli::parse();
    let project = &args.project_name;
    let language = &args.language;
    let config = &load_configuration(project, language);
    create_project(project, config, &args.variant);
}
