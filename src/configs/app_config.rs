use std::path::{Path, PathBuf};
use std::cell::{Ref, RefCell, RefMut};
use ratatui::style::Style;

use crate::layout::Layout as TukaiLayout;

pub struct AppConfig {
  // Path to the storage file
  file_path: PathBuf,

  // Choosen layout
  layout: RefCell<TukaiLayout>,

  // App background is transparent
  pub has_transparent_bg: bool
}

impl AppConfig {
  pub fn default() -> Self {
    Self {
      file_path: PathBuf::from("tukai.bin"),
      layout: RefCell::new(TukaiLayout::default()),
      has_transparent_bg: false
    }
  }

  /// Returns TukaiLayout
  pub fn get_layout(&self) -> Ref<TukaiLayout> {
    self.layout.borrow()
  }

  /// Returns mutable TukaiLayout
  pub fn get_layout_mut(&mut self) -> RefMut<TukaiLayout> {
    self.layout.borrow_mut()
  }

  /// Returns the storage file
  pub fn get_file_path(&self) -> &PathBuf {
    &self.file_path
  }

  /// Returns the background color
  ///
  /// If has_transparent_bg not equals true
  pub fn get_bg_color(&self) -> Style {
    let style = Style::default();
    if self.has_transparent_bg { style } else { style.bg(self.get_layout().get_background_color()) }
  }
}

pub struct AppConfigBuilder {
  // Path to the storage file
  file_path: Option<PathBuf>,

  // Choosen layout
  layout: Option<RefCell<TukaiLayout>>,

  // App background is transparent
  has_transparent_bg: bool
}

impl AppConfigBuilder {
  pub fn new() -> Self {
    Self {
      file_path: None,
      layout: None,
      has_transparent_bg: true
    }
  }

  /// Sets the storage file path
  pub fn file_path<P: AsRef<Path>>(mut self, file_path: P) -> Self {
    self.file_path = Some(file_path.as_ref().to_path_buf());
    self
  }

  /// Sets the layout
  pub fn layout(mut self, layout: TukaiLayout) -> Self {
    self.layout = Some(RefCell::new(layout));
    self
  }

  /// Sets if the app background is transparent
  pub fn has_transparent_bg(mut self, has_transparent_bg: bool) -> Self {
    self.has_transparent_bg = has_transparent_bg;
    self
  }

  pub fn build(self) -> AppConfig {
    let config_default = AppConfig::default();

    AppConfig {
      file_path: self.file_path.unwrap_or(config_default.file_path),
      layout: self.layout.unwrap_or(config_default.layout),
      has_transparent_bg: self.has_transparent_bg
    }
  }
}
