use crossterm::event::{KeyCode, KeyEvent};
use crate::{configs::typing_window_config::TypingWindowConfig, tools::generator::Generator, traits::Window};

pub struct TypingWindow {
  pub generated_text: String,
  pub input: String,
  config: TypingWindowConfig
}

impl Window for TypingWindow {
  fn default() -> Self {
    Self {
      generated_text: Generator::generate_random_text(),
      input: String::new(),
      config: TypingWindowConfig::default()
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

impl TypingWindow {
  #[allow(unused)]
  pub fn config(mut self, config: TypingWindowConfig) -> Self {
    self.config = config;
    self
  }
}
