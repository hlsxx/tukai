use std::path::{Path, PathBuf};
use std::cell::{Ref, RefCell, RefMut};
use ratatui::style::Style;

use serde::{Deserialize, Serialize};
use crate::layout::Layout as TukaiLayout;

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

  /// Toggles background (transparent|layout color)
  pub fn toggle_transparent_bg(&mut self) {
    self.has_transparent_bg = !self.has_transparent_bg;
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

  pub fn build(self) -> AppConfig {
    let config_default = AppConfig::default();

    AppConfig {
      file_path: self.file_path.unwrap_or(config_default.file_path),
      layout: self.layout.unwrap_or(config_default.layout),
      has_transparent_bg: self.has_transparent_bg
    }
  }
}

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
  // pub fn to_string(&self) -> String {
  //   let time = match self {
  //     TypingDuration::Minute => "60s",
  //     TypingDuration::ThirtySec => "30s",
  //     TypingDuration::ThreeMinutes => "180s"
  //   };
  //
  //   time.to_string()
  // }

  pub fn as_seconds(&self) -> usize {
    match self {
      TypingDuration::ThirtySec => 30,
      TypingDuration::Minute => 60,
      TypingDuration::ThreeMinutes => 180
    }
  }
}

pub struct TypingScreenConfig {
  pub typing_duration: TypingDuration,
}

impl Default for TypingScreenConfig {
  fn default() -> Self {
    Self {
      typing_duration: TypingDuration::default()
    }
  }
}

#[allow(unused)]
impl TypingScreenConfig {
  /// Switch the typing duration
  ///
  /// ThirtySec
  /// Minute
  /// ThreeMinutes
  pub fn switch_typing_duration(&mut self) {
    if self.typing_duration == TypingDuration::Minute {
      self.typing_duration = TypingDuration::ThreeMinutes;
    } else if self.typing_duration == TypingDuration::ThreeMinutes {
      self.typing_duration = TypingDuration::ThirtySec
    } else if self.typing_duration == TypingDuration::ThirtySec {
      self.typing_duration = TypingDuration::Minute
    }
  }
}

pub struct TypingScreenConfigBuilder {
  typing_duration: Option<TypingDuration>
}

impl TypingScreenConfigBuilder {
  #[allow(unused)]
  fn time_limit(mut self, typing_duration: TypingDuration) -> Self {
    self.typing_duration = Some(typing_duration);
    self
  }
}

impl ConfigBuilder<TypingScreenConfig> for TypingScreenConfigBuilder {
  fn new() -> Self {
    Self {
      typing_duration: None
    }
  }

  fn build(self) -> TypingScreenConfig {
    let typing_window_config = TypingScreenConfig::default();

    TypingScreenConfig {
      typing_duration: self.typing_duration.unwrap_or(typing_window_config.typing_duration)
    }
  }
}


