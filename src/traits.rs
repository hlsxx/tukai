use crossterm::event::KeyEvent;

pub trait Window {
  fn default() -> Self;
  fn handle_events(&mut self, key: KeyEvent);
}

pub trait ConfigBuilder<T> {
  fn new() -> Self;
  fn build(self) -> T;
}
