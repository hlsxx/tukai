use std::{fs::{self, File}, io::{BufRead, BufReader}, path::PathBuf};
use rand::{seq::SliceRandom, Rng};
use ratatui::{style::Color, widgets::block::Title};

use crate::{config::TypingDuration, layout::LayoutName};

pub struct Generator;

pub fn get_title(layout_name: &LayoutName, screen_name: &str) -> Title<'static> {
  Title::from(format!(
    " tukai v{} 》{} 》{} ",
    env!("CARGO_PKG_VERSION"),
    layout_name,
    screen_name
  ))
}

impl Generator {
  /// Generates a random string of words
  ///
  /// This method generates a string containing random
  /// words from the words/{language}.txt file
  pub fn generate_random_string(
    typing_duration: &TypingDuration,
    language_index: &usize
  ) -> String {
    let words = Words::load_word_files();
    let words = words[language_index].lines().collect::<Vec<&str>>();

    let mut rng = rand::thread_rng();

    let text = words
      .choose_multiple(&mut rng, typing_duration.as_seconds() * 2)
      .fold(String::new(), |mut acc, c| {
        acc.push_str(format!("{} ", c).as_str());
        acc
      });

    text
  }

  /// Generates a random motto for the block bottom title
  pub fn generate_random_motto() -> String {
    let mottos = [
      " Practice today, master tomorrow ",
      " Fingers on keys, progress with ease ",
      " Consistency breeds accuracy ",
      " Type smarter, not harder ",
      " Precision today, perfection tomorrow ",
    ];

    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..mottos.len());

    String::from(mottos[random_index])
  }
}

#[allow(unused)]
pub trait ToDark {
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
  fn to_dark(self) -> Color;
}

impl ToDark for Color {
  fn to_dark(self) -> Color {
    match self {
      Color::Rgb(r, g, b) => {
        let darkened_r = (r as f32 * (1.0 - 0.2)) as u8;
        let darkened_g = (g as f32 * (1.0 - 0.2)) as u8;
        let darkened_b = (b as f32 * (1.0 - 0.2)) as u8;

        Color::Rgb(darkened_r, darkened_g, darkened_b)
      }
      _ => self,
    }
  }
}

pub struct Language {
  // Language files paths from the `words` folder
  language_files: Vec<PathBuf>,

  // Current used language index
  current_index: usize,

  // Current selected language words
  words: Vec<String>
}

impl Language {

  // Creates default empty list of the language files
  pub fn default() -> Self {
    Self {
      language_files: Vec::new(),
      current_index: 0,
      words: Vec::new()
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

  pub fn get_current_index(&self) -> &usize {
    &self.current_index
  }

  pub fn switch_language(&mut self) {
    self.current_index += 1;

    if self.current_index > self.language_files.len() {
      self.current_index = 0;
    }
  }

  /// Returns the paths of all available language files int the `words` folder
  fn load_language_files(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let entries = fs::read_dir("words")?;

    let languages = entries
      .filter_map(|entry| entry.ok())
      .filter(|entry| entry.path().is_file())
      .map(|entry| entry.path())
      .collect::<Vec<PathBuf>>();

    Ok(languages)
  }

  fn load_language_words(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let language_file_path = self.language_files.get(self.current_index)
      .ok_or("Not found a language dictionary file")?;

    let file = File::open(&language_file_path)?;
    let buffer = BufReader::new(file);

    let words = buffer
      .lines()
      .filter_map(|line| line.ok())
      .flat_map(|line| line.split_whitespace().map(String::from).collect::<Vec<String>>()) // Split into words
      .collect::<Vec<String>>();
    
    Ok(words)
  }

}
