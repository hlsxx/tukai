use serde::Deserialize;

#[derive(Deserialize)]
#[allow(unused)]
pub struct Package {
  pub version: String
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct Config {
  pub package: Package
}
