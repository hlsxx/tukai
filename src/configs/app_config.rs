use std::{borrow::BorrowMut, path::{Path, PathBuf}};
use crate::layout::Layout as TukaiLayout;

pub struct AppConfig {
  file_path: PathBuf,
  layout: TukaiLayout
}

impl AppConfig {
  pub fn default() -> Self {
    Self {
      file_path: PathBuf::from("tukai.bin"),
      layout: TukaiLayout::default()
    }
  }

  pub fn get_layout(&self) -> &TukaiLayout {
    &self.layout
  }

  pub fn get_layout_mut(&mut self) -> &mut TukaiLayout {
    self.layout.borrow_mut()
  }

  pub fn get_file_path(&self) -> &PathBuf {
    &self.file_path
  }
}

pub struct AppConfigBuilder {
  file_path: Option<PathBuf>,
  layout: Option<TukaiLayout>
}

impl AppConfigBuilder {
  pub fn new() -> Self {
    Self {
      file_path: None,
      layout: None
    }
  }

  pub fn file_path<P: AsRef<Path>>(mut self, file_path: P) -> Self {
    self.file_path = Some(file_path.as_ref().to_path_buf());
    self
  }

  pub fn layout(mut self, layout: TukaiLayout) -> Self {
    self.layout = Some(layout);
    self
  }

  pub fn build(self) -> AppConfig {
    let config_default = AppConfig::default();

    AppConfig {
      file_path: self.file_path.unwrap_or(config_default.file_path),
      layout: self.layout.unwrap_or(config_default.layout)
    }
  }
}
