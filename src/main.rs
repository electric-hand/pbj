use clap::Parser;
use pbj::toml::config::Config;
use pbj::parser::{Cli, Commands};
use pbj::commands::generate::generate;

fn main() {
    colog::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { prefix, project_name, template, variant } => {
            let config = Config::load();
            let template_key = config.get_template_key(&template);
            let prefix = config.get_prefix(&prefix);
            let variant = config.get_variant(&variant);
            generate(&prefix, &project_name, &template_key, &variant);
        },
        Commands::BuiltIns => {
            todo!()
        }
    }


}
