use crossterm::event::{KeyCode, KeyEvent};
use crate::traits::Window;

pub struct SearchWindow {
  pub input: String
}

impl Window for SearchWindow {
  fn new() -> Self {
    Self {
      input: String::new()
    }
  }

  fn handle_events(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Char(c) => self.input.push(c),
      KeyCode::Backspace => { let _ = self.input.pop(); },
      // KeyCode::Enter => is_loading = !is_loading,
      _ => ()
    }
  }
}
