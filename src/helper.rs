use rand::{seq::SliceRandom, Rng};
use ratatui::{
  widgets::block::Title,
  style::Color
};

use crate::layout::LayoutName;

pub struct Generator;

pub fn get_title(layout_name: &LayoutName, screen_name: &str) -> Title<'static> {
  Title::from(format!(" tukai v{} 》{} 》{} ", env!("CARGO_PKG_VERSION"), layout_name, screen_name))
}

impl Generator {

  /// Generates a random string of words
  ///
  /// This method generates a string containing random
  /// words from the words/en.txt file
  pub fn generate_random_string(amount: usize) -> String {
    let words_string = include_str!("../words/en.txt");

    let words = words_string.lines()
      .map(|line| line)
      .collect::<Vec<&str>>();

    let mut rng = rand::thread_rng();

    let text = words.choose_multiple(&mut rng, amount * 2)
      .fold(String::new(), |mut acc, c| {
        acc.push_str(format!("{} ", c).as_str());
        acc
      });

    text
  }

  /// Generates a random motto for the block bottom title
  pub fn generate_random_motto() -> String {
    let mottos = vec![
      " Practice today, master tomorrow ",
      " Fingers on keys, progress with ease ",
      " Consistency breeds accuracy ",
      " Type smarter, not harder ",
      " Precision today, perfection tomorrow "
    ];

    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..mottos.len());
  
    String::from(mottos[random_index])
  }

}

#[allow(unused)]
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
      },
      _ => self
    }
  }
}
