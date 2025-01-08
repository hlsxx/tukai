use std::path::{Path, PathBuf};
use std::cell::{Ref, RefCell, RefMut};
use ratatui::style::Style;

use serde::{Deserialize, Serialize};
use crate::layout::Layout as TukaiLayout;
use crate::storage::storage_handler::StorageData;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Debug, Clone)]
/// Represents the available durations for the test
///
/// This enum defines default durations
///
/// # Variants
/// - `ThirtySec` - 30 seconds typing duration
/// - `Minute` - 60 seconds typing duration
/// - `ThreeMinutes` - 180 seconds typing duration
pub enum TypingDuration {
  ThirtySec,
  Minute,
  ThreeMinutes
}

impl Default for TypingDuration {
  fn default() -> Self {
    Self::Minute
  }
}

impl TypingDuration {
  pub fn as_seconds(&self) -> usize {
    match self {
      TypingDuration::ThirtySec => 30,
      TypingDuration::Minute => 60,
      TypingDuration::ThreeMinutes => 180
    }
  }
}

#[allow(unused)]
pub trait ConfigBuilder<T> {
  fn new() -> Self;
  fn build(self) -> T;
}

pub struct AppConfig {
  // Path to the storage file
  file_path: PathBuf,

  // Choosen layout
  layout: RefCell<TukaiLayout>,

  // App background is transparent
  pub has_transparent_bg: bool,

  // Typing duration
  pub typing_duration: TypingDuration
}

impl AppConfig {
  pub fn default() -> Self {
    Self {
      file_path: PathBuf::from("tukai.bin"),
      layout: RefCell::new(TukaiLayout::default()),
      has_transparent_bg: false,
      typing_duration: TypingDuration::default()
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

  /// Returns mutable TypingDuration
  // pub fn get_typing_duration(&mut self) -> RefMut<TypingDuration> {
  //   self.typing_duration.borrow_mut()
  // }

  /// Returns the storage file
  pub fn get_file_path(&self) -> &PathBuf {
    &self.file_path
  }

  /// Toggles a background (transparent | layout color)
  ///
  /// Returns a current state
  pub fn toggle_transparent_bg(&mut self) -> bool {
    self.has_transparent_bg = !self.has_transparent_bg;
    self.has_transparent_bg
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
  has_transparent_bg: bool,

  // Typing duration
  typing_duration: Option<TypingDuration>,
}

#[allow(unused)]
impl AppConfigBuilder {
  pub fn new() -> Self {
    Self {
      file_path: None,
      layout: None,
      has_transparent_bg: true,
      typing_duration: None
    }
  }

  /// Creates Config from a storage data
  // pub fn from(storage_data: StorageData) -> Self {
  //   Self {
  //     file_path: None,
  //     layout: NO,
  //     typing_duration: storage_data.1,
  //     has_transparent_bg: storage_data.2,
  //   }
  // }

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

  pub fn build(self) -> AppConfig {
    let config_default = AppConfig::default();

    AppConfig {
      file_path: self.file_path.unwrap_or(config_default.file_path),
      layout: self.layout.unwrap_or(config_default.layout),
      has_transparent_bg: self.has_transparent_bg,
      typing_duration: self.typing_duration.unwrap_or(config_default.typing_duration)
    }
  }
}
