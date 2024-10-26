use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::{Color, Style}, text::{Line, Span, Text}, widgets::Paragraph};
use crate::{configs::typing_window_config::TypingWindowConfig, tools::generator::Generator, traits::Window};

pub struct TypingWindow {
  pub generated_text: String,
  pub input: String,

  cursor_index: usize,
  config: TypingWindowConfig
}

impl Window for TypingWindow {
  fn default() -> Self {
    Self {
      generated_text: Generator::generate_random_string(),
      input: String::new(),

      cursor_index: 0,
      config: TypingWindowConfig::default()
    }
  }

  fn handle_events(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Char(c) => {
        self.input.push(c);
        self.cursor_index += 1;
      },
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

  pub fn get_paragraph(&mut self) -> Paragraph {
    let mut lines = Vec::new();

    let line = self.generated_text.chars().enumerate().map(|(i, c)| {
      if i == self.cursor_index {
        Span::styled(c.to_string(), Style::default().fg(Color::Black).bg(Color::White))
      } else if i < self.cursor_index {
        if self.input.chars().nth(i) == Some(c) {
          Span::styled(c.to_string(), Style::default().fg(Color::from_u32(0x805CBF)))
        } else {
          Span::styled(c.to_string(), Style::default().fg(Color::Red))
        }
      } else {
        Span::styled(c.to_string(), Style::default().fg(Color::White))
      }
    }).collect();

    lines.push(line);

    let text = Text::from(lines);

    Paragraph::new(text)
  }
}
