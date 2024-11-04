use std::{error, fs};

use serde::Deserialize;
use toml;

#[derive(Deserialize)]
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
    let tukaj_toml_string = fs::read_to_string("Cargo.toml")?;
    let tukaj_config = toml::from_str::<Config>(&tukaj_toml_string)?;

    Ok(tukaj_config.package)
  }
}
