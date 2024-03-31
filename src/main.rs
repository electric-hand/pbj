use clap::Parser;
use log::info;
use pbj::config_toml::load_config;
use pbj::constants::{DEFAULT_PREFIX, DEFAULT_PREFIX_SEPARATOR, DEFAULT_TEMPLATE, DEFAULT_VARIANT};
use pbj::parser::Cli;
use pbj::project_builder::create_project;
use pbj::template_toml::load_project_template;

fn main() {
    colog::init();

    let args = Cli::parse();
    let project_name = args.project_name;

    let config = load_config();

    let template_key = args
        .template
        .unwrap_or(config.template.unwrap_or(DEFAULT_TEMPLATE.to_string()));
    let mut prefix = args.prefix.unwrap_or(DEFAULT_PREFIX.to_string());
    let prefix_separator = config
        .prefix_separator
        .unwrap_or(DEFAULT_PREFIX_SEPARATOR.to_string());
    let variant = args
        .variant
        .unwrap_or(config.variant.unwrap_or(DEFAULT_VARIANT.to_string()));

    let prefix = match prefix.as_str() {
        "" => "",
        _ => {
            prefix.push_str(&prefix_separator);
            &prefix
        }
    };

    let template = load_project_template(&project_name, &template_key);
    create_project(&project_name, &prefix, &template, &variant);
    info!(
        "Successfully created your {} project {}.  Happy Coding!",
        template_key, project_name
    );
}
