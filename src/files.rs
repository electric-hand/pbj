use std::{fs::read_to_string, path::PathBuf};

use log::{error, info};

const PYTHON_TOML: &str = include_str!("../templates/python.toml");
const TYPESCRIPT: &str = include_str!("../templates/typescript.toml");
const CONFIG: &str = include_str!("../default_config.toml");
const CONFIG_DIR: &str = ".config";

const APP_DIR: &str = "pbj";
const TEMPLATE_DIR: &str = "templates";

fn get_template_path(template: &str) -> PathBuf {
    let template = format!("{}.toml", template);
    PathBuf::from_iter(vec![TEMPLATE_DIR.to_string(), template].iter())
}

pub fn get_default_file_contents(key: &str) -> Option<&str> {

    let result = match key {
        "python" => Some(PYTHON_TOML),
        "typescript" => Some(TYPESCRIPT),
        "config" => Some(CONFIG),
        _ => None,
    };

    if result.is_some() {
        info!("unable to find file for \"{key}\", found and using contents from built-in default");
    } else {
        error!("unable to find file for \"{key}\"and no built-in contents found");
    };

    result
}

pub fn get_file(file: &PathBuf) -> Option<String> {
    let dot_config_path = dirs::home_dir()?.join(CONFIG_DIR).join(APP_DIR).join(&file);

    let local_os_config_path = dirs::config_local_dir()?.join(APP_DIR).join(&file);

    let os_config_path = dirs::config_dir()?.join(APP_DIR).join(&file);

    if let Some(contents) = read_to_string(&dot_config_path).ok() {
        print_loading(&dot_config_path);
        return Some(contents);
    }

    if let Some(contents) = read_to_string(&local_os_config_path).ok() {
        print_loading(&local_os_config_path);
        return Some(contents);
    }

    if let Some(contents) = read_to_string(&os_config_path).ok() {
        print_loading(&os_config_path);
        return Some(contents);
    }

    None
}

fn print_loading(path: &PathBuf) {
    info!("loading file from path {:?}", path);
}

pub fn get_template(template_name: &str) -> Option<String> {
    let template = get_template_path(template_name);
    if let Some(contents) = get_file(&template) {
        return Some(contents);
    }
    if let Some(contents) = get_default_file_contents(&template_name) {
        return Some(contents.to_string());
    }
    None
}
