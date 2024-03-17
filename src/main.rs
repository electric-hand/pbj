use clap::Parser;
use pbj::config::load_configuration;
use pbj::parser::Cli;
use pbj::project_creator::create_project;

fn main() {
    let args = Cli::parse();
    let project_name = &args.project_name;
    let language = &args.language;
    let prefix = &args.prefix;
    let config = load_configuration(project_name, language);
    create_project(project_name, prefix, &config, &args.variant);
    println!();
    println!("Successfully created your {} project {}.  Happy Coding!", language, project_name);
}
