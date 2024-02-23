use tddme::TestDrivenConfig;
use std::fs;

fn main() {
    let toml_file = fs::read_to_string("./explore/python.toml").expect("this to work");
    let decoded: TestDrivenConfig = toml::from_str(&toml_file).expect("bad toml");
    println!("{:#?}", decoded);
}
