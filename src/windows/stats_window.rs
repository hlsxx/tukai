use crossterm::event::{KeyCode, KeyEvent};
use std::env;
use crate::traits::Window;

pub struct StatsWindow {
  pub input: String
}

impl Window for StatsWindow {
  fn default() -> Self {
    let path = env::current_dir().expect("Error getting current path");

    Self {
      input: path.to_string_lossy().into_owned()
    }
  }

  fn handle_events(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Char(c) => self.input.push(c),
      KeyCode::Backspace => { let _ = self.input.pop(); },
      _ => ()
    }
  }
}
