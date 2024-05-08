use clap::Parser;
use pbj::toml::config::load_config;
use pbj::constants::{DEFAULT_PREFIX, DEFAULT_PREFIX_SEPARATOR, DEFAULT_TEMPLATE, DEFAULT_VARIANT_VALUE};
use pbj::parser::{Cli, Commands};
use pbj::commands::generate::create_project;
use pbj::toml::template::load_project_template;

fn main() {
    colog::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { prefix, project_name, template, variant } => {
            let config = load_config();

            let template_key = template
                .unwrap_or(config.template.unwrap_or(DEFAULT_TEMPLATE.to_string()));
            let mut prefix = prefix.unwrap_or(DEFAULT_PREFIX.to_string());
            let prefix_separator = config
                .prefix_separator
                .unwrap_or(DEFAULT_PREFIX_SEPARATOR.to_string());
            let variant = variant
                .unwrap_or(config.variant.unwrap_or(DEFAULT_VARIANT_VALUE.to_string()));
        
            let prefix = match prefix.as_str() {
                "" => "",
                _ => {
                    prefix.push_str(&prefix_separator);
                    &prefix
                }
            
            };
        
            let template = load_project_template(&project_name, &template_key);
            create_project(&project_name, &prefix, &template, &variant);
        },
        Commands::BuiltIns => {
            todo!()
        }
    }


}
