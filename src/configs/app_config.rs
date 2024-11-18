use std::path::{Path, PathBuf};
use std::cell::{Ref, RefCell, RefMut};
use crate::layout::Layout as TukaiLayout;

pub struct AppConfig {
  file_path: PathBuf,
  layout: RefCell<TukaiLayout>
}

impl AppConfig {
  pub fn default() -> Self {
    Self {
      file_path: PathBuf::from("tukai.bin"),
      layout: RefCell::new(TukaiLayout::default())
    }
  }

  pub fn get_layout(&self) -> Ref<TukaiLayout> {
    self.layout.borrow()
  }

  pub fn get_layout_mut(&mut self) -> RefMut<TukaiLayout> {
    self.layout.borrow_mut()
  }

  pub fn get_file_path(&self) -> &PathBuf {
    &self.file_path
  }
}

pub struct AppConfigBuilder {
  file_path: Option<PathBuf>,
  layout: Option<RefCell<TukaiLayout>>
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
    self.layout = Some(RefCell::new(layout));
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
