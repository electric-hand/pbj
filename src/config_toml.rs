#![allow(dead_code)]
use std::path::PathBuf;

use serde::Deserialize;

use crate::constants::CONFIG_FILE_NAME;
use crate::files::{get_default_file_contents, get_file};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub template: Option<String>,
    pub prefix_separator: Option<String>,
    pub variant: Option<String>,
}

pub fn load_config() -> Config {
    if let Some(config) = get_file(&PathBuf::from(CONFIG_FILE_NAME)) {
        toml::from_str(&config).expect("toml parsing failed.")
    } else {
        toml::from_str(
            get_default_file_contents(CONFIG_FILE_NAME)
                .expect("default config should always be there"),
        )
        .expect("parsing of default config file failed.  This should never happen!")
    }
}
