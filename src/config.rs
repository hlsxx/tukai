use std::{error, fs};

use serde::Deserialize;
use toml;

#[derive(Deserialize)]
#[allow(unused)]
pub struct Package {
  version: String
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct Config {
  package: Package
}

impl Config {
  pub fn get_package() -> Result<Package, Box<dyn error::Error>> {
    let tukai_toml_string = fs::read_to_string("Cargo.toml")?;
    let tukai_config = toml::from_str::<Config>(&tukai_toml_string)?;

    Ok(tukai_config.package)
  }
}
