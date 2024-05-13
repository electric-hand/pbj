#![allow(dead_code)]
use std::path::PathBuf;

use serde::Deserialize;

use crate::constants::{
    CONFIG_FILE_NAME, DEFAULT_PREFIX_SEPARATOR, DEFAULT_TEMPLATE,
    DEFAULT_VARIANT_VALUE,
};
use crate::files::{get_default_file_contents, read_file};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub template: Option<String>,
    pub prefix_separator: Option<String>,
    pub variant: Option<String>,
}

impl Config {
    pub fn get_template_key(&self, template: &Option<String>) -> String {
        let template = match template.as_ref() {
            Some(template) => template,
            None => DEFAULT_TEMPLATE
        };
        template.to_owned()
    }

    pub fn get_prefix(&self, prefix: &Option<String>) -> String {
        let prefix_separator = match self.prefix_separator.as_ref() {
            Some(separator) => separator,
            None => DEFAULT_PREFIX_SEPARATOR
        };

        let prefix = match prefix.as_ref() {
            Some(prefix) => [prefix, prefix_separator].concat(),
            None => "".to_owned()
        };
        prefix
    }

    pub fn get_variant(&self, variant: &Option<String>) -> String {
        let variant = match variant.as_ref() {
            Some(variant) => variant,
            None => DEFAULT_VARIANT_VALUE
        };
        variant.to_owned()
    }

    pub fn load() -> Self {
        if let Some(config) = read_file(&PathBuf::from(CONFIG_FILE_NAME)) {
            toml::from_str(&config).expect("toml parsing failed.")
        } else {
            toml::from_str(
                get_default_file_contents(CONFIG_FILE_NAME)
                    .expect("default config should always be there"),
            )
            .expect("parsing of default config file failed.  This should never happen!")
        }
    }
}
