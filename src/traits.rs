use crossterm::event::KeyEvent;
use ratatui::{
  layout::Rect, style::Color, Frame
};

pub trait Window {
  fn default() -> Self;
  fn handle_events(&mut self, key: KeyEvent);
  fn render(&self, frame: &mut Frame, area: Rect);

  fn is_active(&self) -> bool;

  fn get_border_color(&self) -> Color {
    if self.is_active() {
      Color::from_u32(0x805CBF)
    } else {
      Color::from_u32(0x00999999)
    }
  }
}

pub trait ConfigBuilder<T> {
  fn new() -> Self;
  fn build(self) -> T;
}
