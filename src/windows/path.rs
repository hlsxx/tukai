use crossterm::event::{KeyCode, KeyEvent};
use std::env;
use crate::traits::Window;

pub struct PathWindow {
  pub input: String
}

impl Window for PathWindow {
  fn new() -> Self {
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
