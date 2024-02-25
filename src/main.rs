use std::fs;
use clap::Parser;

use crate::config::TestDrivenConfig;
use crate::parser::Cli;

pub mod config;
pub mod parser;


fn main() {

    let args = Cli::parse();
    let language = args.language;
    let path = format!("./templates/{language}.toml");
    println!("{}", path);
    let toml_file = fs::read_to_string(path).expect("this to work");
    let decoded: TestDrivenConfig = toml::from_str(&toml_file).expect("bad toml");
    println!("{:#?}", decoded);
}
