use crossterm::event::KeyEvent;
use ratatui::{
  layout::Rect, style::Color, Frame
};

use crate::{constants::colors, helper::get_color_rgb, layout::Layout as TukajLayout};

pub trait Window {
  fn default() -> Self;

  /// Handle events
  /// Returns `true` if event is consumed
  fn handle_events(&mut self, key: KeyEvent) -> bool;

  /// Window is currently active
  fn is_active(&self) -> bool;
  fn toggle_active(&mut self);

  /// Get border color (active/nonactive)
  fn get_border_color(&self) -> Color {
    let color_rgb = if self.is_active() { colors::PRIMARY } else { colors::SECONDARY };
    get_color_rgb(color_rgb)
  }

  /// Render window
  fn render(&self, frame: &mut Frame, layout: &TukajLayout, area: Rect);
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

