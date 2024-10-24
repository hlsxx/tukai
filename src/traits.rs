use crossterm::event::KeyEvent;

pub trait Window {
  fn new() -> Self;
  fn handle_events(&mut self, key: KeyEvent);
}
