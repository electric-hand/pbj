use std::{collections::HashMap, fs::read_to_string};

const PYTHON_TOML: &str = include_str!("../templates/python.toml");
const TYPESCRIPT: &str = include_str!("../templates/typescript.toml");
const CONFIG_DIR_NAME: &str = "pbj";

fn get_default_templates() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("python.toml", PYTHON_TOML),
        ("typescript.toml", TYPESCRIPT),
    ])
}

pub fn get_template(template_name: &str) -> Option<String> {
    let template_name = format!("{}.toml", template_name);
    let config_local_template = dirs::config_local_dir()?
        .join(CONFIG_DIR_NAME)
        .join(&template_name);
    let config_template = dirs::config_dir()?
        .join(CONFIG_DIR_NAME)
        .join(&template_name);

    if let Some(template) = read_to_string(&config_local_template).ok() {
        return Some(template);
    }

    if let Some(template) = read_to_string(&config_template).ok() {
        return Some(template);
    }

    if let Some(&default_template) = get_default_templates().get(template_name.as_str()) {
        return Some(default_template.to_string());
    }
    None
}