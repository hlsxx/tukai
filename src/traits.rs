use crossterm::event::KeyEvent;
use ratatui::{
  layout::Rect, style::Color, Frame
};

use crate::{constants::colors, helper::get_color_rgb};

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
  fn render(&self, frame: &mut Frame, area: Rect);
}

pub trait ConfigBuilder<T> {
  fn new() -> Self;
  fn build(self) -> T;
}
