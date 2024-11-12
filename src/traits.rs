use color_eyre::config::Theme;
use crossterm::event::KeyEvent;
use ratatui::{
  layout::Rect, style::Color, Frame
};

use crate::layout::Layout as TukaiLayout;

pub trait Window {
  fn default() -> Self;

  /// Handle events
  /// Returns `true` if event is consumed
  fn handle_events(&mut self, key: KeyEvent) -> bool;

  /// Window is currently active
  fn is_active(&self) -> bool;
  fn toggle_active(&mut self);

  /// After another window switched
  fn hide(&mut self) {
    self.toggle_active();
  }

  /// Render window instructions
  fn render_instructions(&self, frame: &mut Frame, layout: &TukaiLayout, area: Rect);

  /// Render window
  fn render(&self, frame: &mut Frame, layout: &TukaiLayout, area: Rect);

}

#[allow(unused)]
pub trait ConfigBuilder<T> {
  fn new() -> Self;
  fn build(self) -> T;
}

#[allow(unused)]
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
        let darkened_r = (r as f32 * (1.0 - 0.5)) as u8;
        let darkened_g = (g as f32 * (1.0 - 0.5)) as u8;
        let darkened_b = (b as f32 * (1.0 - 0.5)) as u8;

        Color::Rgb(darkened_r, darkened_g, darkened_b)
      },
      _ => self
    }
  }
}


