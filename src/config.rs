use ratatui::style::Style;
use rust_embed::RustEmbed;
use std::cell::{Ref, RefCell, RefMut};

use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
use std::{collections::HashMap, fmt::Display, hash::Hash};

use maplit::hashmap;
use ratatui::style::Color;

pub trait ToColor {
  /// Converts the `(u8, u8, u8)` tuple to a `Color::Rgb`
  ///
  /// # Example
  ///
  /// ```
  /// use ratatui::style::Color
  ///
  /// let rgb: (u8, u8, u8) = (128, 64, 255);
  /// let color = rgb.to_color();
  ///
  /// assert_eq!(color, Color::Rgb(128, 64, 255));
  /// ```
  fn to_color(self) -> Color;
}

/// Type alias for representing an RGB color as a tuple
type RgbColor = (u8, u8, u8);

impl ToColor for RgbColor {
  fn to_color(self) -> Color {
    Color::Rgb(self.0, self.1, self.2)
  }
}

#[allow(dead_code)]
pub enum TukaiLayoutColorTypeEnum {
  Primary,
  Secondary,
  Text,
  TextReverse,
  Background,
  Error,
}

/// Layout name for Tukai application
/// Used for a switchable layout colors
///
/// Switchable with a `ctrl-s` shortcut
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum TukaiLayoutName {
  Iced,
  Rust,
  Anime,
  Deadpool,
  Wolverine,
  Goblin,
}

/// Display used in the Tukai paragraph block_title
impl Display for TukaiLayoutName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use TukaiLayoutName::*;

    let display_text = match self {
      Iced => "🥶 Iced",
      Rust => "🦀 Rust",
      Anime => "🌸 Anime",
      Deadpool => "🩸🔞 Deadpool",
      Wolverine => "💪🍺 Wolverine",
      Goblin => "🌳 Goblin",
    };

    write!(f, "{}", display_text)
  }
}

/// Set of the colors used in the application.
pub struct TukaiLayoutColors {
  primary: RgbColor,
  text: RgbColor,
  text_current: RgbColor,
  text_current_bg: RgbColor,
  background: RgbColor,
  error: RgbColor,
}

impl TukaiLayoutColors {
  pub fn new(
    primary: RgbColor,
    text: RgbColor,
    text_current: RgbColor,
    text_current_bg: RgbColor,
    background: RgbColor,
    error: RgbColor,
  ) -> Self {
    Self {
      primary,
      text,
      text_current,
      text_current_bg,
      background,
      error,
    }
  }
}

/// Tukai layout includes all layouts
/// also, contains `transitions`, and the current selected layout name
pub struct TukaiLayout {
  // Set of the layouts
  layouts: HashMap<TukaiLayoutName, TukaiLayoutColors>,

  // Rules for switchable transition of the layout
  transitions: HashMap<TukaiLayoutName, TukaiLayoutName>,

  // Current selected layout name
  active_layout_name: TukaiLayoutName,
}

impl TukaiLayout {
  pub fn default() -> Self {
    use TukaiLayoutName::*;

    let layouts = hashmap! {
      Iced => {
        TukaiLayoutColors::new(
         (108, 181, 230),
         (232, 232, 232),
         (25, 74, 107),
         (200, 200, 200),
         (37, 40, 46),
         (214, 90, 90),
        )
      },
      Anime => {
        TukaiLayoutColors::new(
          (152, 117, 201),
          (222, 135, 174),
          (49, 45, 51),
          (222, 170, 146),
          (31, 27, 30),
          (227, 138, 138),
        )
      },
      Deadpool => {
        TukaiLayoutColors::new(
          (139, 35, 35),
          (210, 210, 210),
          (23, 23, 23),
          (210, 210, 210),
          (33, 29, 29),
          (110, 110, 110),
        )
      },
      Wolverine => {
        TukaiLayoutColors::new(
          (196, 166, 51),
          (200, 200, 200),
          (23,23,23),
          (210, 210, 210),
          (10, 14, 18),
          (110, 110, 110),
        )
      },
      Rust => {
        TukaiLayoutColors::new(
          (150, 63, 17),
          (255, 178, 137),
          (255, 178, 137),
          (150, 63, 17),
          (24, 8, 2),
          (120, 120, 120),
        )
      },
      Goblin => {
        TukaiLayoutColors::new(
          (82, 140, 25),
          (136, 207, 66),
          (220, 220, 220),
          (39, 61, 17),
          (32, 36, 30),
          (117, 71, 56),
        )
      }
    };

    // Define transtions for switch order
    let transitions = HashMap::from([
      (Iced, Anime),
      (Anime, Deadpool),
      (Deadpool, Wolverine),
      (Wolverine, Rust),
      (Rust, Goblin),
      (Goblin, Iced),
    ]);

    Self {
      layouts,
      transitions,
      active_layout_name: TukaiLayoutName::Iced,
    }
  }

  /// Returns the currect active layout name
  pub fn get_active_layout_name(&self) -> &TukaiLayoutName {
    &self.active_layout_name
  }

  /// Sets a new active layout name
  pub fn active_layout_name(&mut self, active_layout_name: TukaiLayoutName) {
    self.active_layout_name = active_layout_name;
  }

  /// Switches to a next layout, then returns that layout
  ///
  /// Check `self.transitions`.
  pub fn switch_to_next_layout(&mut self) -> TukaiLayoutName {
    if let Some(next_layout_name) = self.transitions.get(&self.active_layout_name) {
      self.active_layout_name = next_layout_name.clone();
    };

    self.active_layout_name.clone()
  }

  fn get_layout_colors(&self) -> &TukaiLayoutColors {
    self.layouts.get(&self.active_layout_name).unwrap()
  }

  pub fn get_primary_color(&self) -> Color {
    self.get_layout_colors().primary.to_color()
  }

  pub fn get_text_color(&self) -> Color {
    self.get_layout_colors().text.to_color()
  }

  pub fn get_text_current_color(&self) -> Color {
    self.get_layout_colors().text_current.to_color()
  }

  pub fn get_text_current_bg_color(&self) -> Color {
    self.get_layout_colors().text_current_bg.to_color()
  }

  pub fn get_error_color(&self) -> Color {
    self.get_layout_colors().error.to_color()
  }

  pub fn get_background_color(&self) -> Color {
    self.get_layout_colors().background.to_color()
  }
}

#[derive(RustEmbed)]
#[folder = "dictionary/"]
struct LanguageDictionary;

pub struct Language {
  // Language files paths from the `words` folder
  language_files: Vec<String>,

  // Current used language index
  current_index: usize,

  // Current selected language words
  words: Vec<String>,
}

impl Language {
  // Creates default empty list of the language files
  pub fn default() -> Self {
    Self {
      language_files: Vec::new(),
      current_index: 0,
      words: Vec::new(),
    }
  }

  /// Load language files from the `words` folder
  pub fn init(mut self) -> Self {
    if let Ok(language_files) = self.load_language_files() {
      self.language_files = language_files;
    }

    // If language dictionary files were founded
    // Sets the words
    if self.language_files.len() > 0 {
      if let Ok(words) = self.load_language_words() {
        self.words = words;
      }
    }

    self
  }

  pub fn current_index(&mut self, index: usize) {
    self.current_index = index;
  }

  #[allow(unused)]
  pub fn get_current_index(&self) -> &usize {
    &self.current_index
  }

  /// Switches a current language
  pub fn switch_language(&mut self) -> usize {
    self.current_index += 1;

    if self.current_index >= self.language_files.len() {
      self.current_index = 0;
    }

    self.current_index.clone()
  }

  /// Returns the paths of all available language files in the `words` folder.
  ///
  /// So i.e. available languages
  pub fn load_language_files(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let languages = LanguageDictionary::iter()
      .map(|file| file.to_string())
      .collect::<Vec<String>>();

    // Maybe in feature manual load custom languages
    //
    // let languages = entries
    //   .filter_map(|entry| entry.ok())
    //   .filter(|entry| entry.path().is_file())
    //   .map(|entry| entry.path())
    //   .collect::<Vec<PathBuf>>();

    Ok(languages)
  }

  /// Returns current selected languages words from the language file
  ///
  /// So i.e. language words
  pub fn load_language_words(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let language_file_path = self
      .language_files
      .get(self.current_index)
      .ok_or("Not found a language dictionary file")?;

    let file = LanguageDictionary::get(language_file_path).unwrap();

    let words = std::str::from_utf8(&file.data)?
      .lines()
      .flat_map(|line| {
        line
          .split_whitespace()
          .map(String::from)
          .collect::<Vec<String>>()
      })
      .collect::<Vec<String>>();

    Ok(words)
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
  ThreeMinutes,
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
      TypingDuration::ThreeMinutes => 180,
    }
  }
}

#[allow(unused)]
pub trait ConfigBuilder<T> {
  fn new() -> Self;
  fn build(self) -> T;
}

pub struct TukaiConfig {
  // Path to the storage file
  file_path: PathBuf,

  // Choosen layout
  layout: RefCell<TukaiLayout>,

  // Current language
  language: RefCell<Language>,

  // App background is transparent
  pub has_transparent_bg: bool,

  // Typing duration
  pub typing_duration: TypingDuration,
}

impl TukaiConfig {
  pub fn default() -> Self {
    Self {
      file_path: PathBuf::from("tukai.bin"),
      layout: RefCell::new(TukaiLayout::default()),
      language: RefCell::new(Language::default().init()),
      has_transparent_bg: false,
      typing_duration: TypingDuration::default(),
    }
  }

  /// Returns `TukaiLayout`
  pub fn get_layout(&self) -> Ref<TukaiLayout> {
    self.layout.borrow()
  }

  /// Returns `Language`
  pub fn get_language(&self) -> Ref<Language> {
    self.language.borrow()
  }

  /// Returns mutable TukaiLayout
  pub fn get_layout_mut(&mut self) -> RefMut<TukaiLayout> {
    self.layout.borrow_mut()
  }

  /// Returns mutable TukaiLayout
  pub fn get_language_mut(&mut self) -> RefMut<Language> {
    self.language.borrow_mut()
  }

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

  /// Switches the typing duration.
  ///
  /// Options:
  /// 1. Minute
  /// 2. Three minutes
  /// 3. Thirty seconds
  pub fn switch_typing_duration(&mut self) -> TypingDuration {
    self.typing_duration = match self.typing_duration {
      TypingDuration::Minute => TypingDuration::ThreeMinutes,
      TypingDuration::ThreeMinutes => TypingDuration::ThirtySec,
      TypingDuration::ThirtySec => TypingDuration::Minute,
    };

    self.typing_duration.clone()
  }

  /// Returns the background color
  ///
  /// If has_transparent_bg not equals true
  pub fn get_bg_color(&self) -> Style {
    let style = Style::default();
    if self.has_transparent_bg {
      style
    } else {
      style.bg(self.get_layout().get_background_color())
    }
  }
}

pub struct TukaiConfigBuilder {
  // Path to the storage file
  file_path: Option<PathBuf>,

  // Selected layout
  layout: Option<RefCell<TukaiLayout>>,

  // Selected language
  language: Option<RefCell<Language>>,

  // App background is transparent
  has_transparent_bg: bool,

  // Typing duration
  typing_duration: Option<TypingDuration>,
}

#[allow(unused)]
impl TukaiConfigBuilder {
  pub fn new() -> Self {
    Self {
      file_path: None,
      layout: None,
      language: None,
      has_transparent_bg: true,
      typing_duration: None,
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

  /// Builds and returns `TukaiConfig`
  pub fn build(self) -> TukaiConfig {
    let config_default = TukaiConfig::default();

    TukaiConfig {
      file_path: self.file_path.unwrap_or(config_default.file_path),
      layout: self.layout.unwrap_or(config_default.layout),
      language: self.language.unwrap_or(config_default.language),
      has_transparent_bg: self.has_transparent_bg,
      typing_duration: self
        .typing_duration
        .unwrap_or(config_default.typing_duration),
    }
  }
}
