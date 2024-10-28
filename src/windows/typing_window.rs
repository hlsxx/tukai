use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
  layout::{Alignment, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span, Text},
  widgets::{block::{Position, Title}, Block, BorderType, Borders, Padding, Paragraph},
  Frame
};

use crate::{
  configs::typing_window_config::TypingWindowConfig, constants::{self, colors}, helper::get_color_rgb, tools::generator::Generator, traits::Window
};


#[derive(Clone, Copy)]
pub struct Stats {
  pub errors_count: usize,
}

impl Default for Stats {
  fn default() -> Self {
    Self {
      errors_count: 0
    }
  }
}

pub struct TypingWindow {
  pub generated_text: String,
  pub input: String,

  pub stats: Stats,

  pub is_active: bool,

  cursor_index: usize,
  previous_index: usize,

  config: TypingWindowConfig
}

impl Window for TypingWindow {
  fn default() -> Self {
    Self {
      generated_text: Generator::generate_random_string(50),
      input: String::new(),

      stats: Stats::default(),

      is_active: false,

      cursor_index: 0,
      previous_index: 0,

      config: TypingWindowConfig::default()
    }
  }

  fn is_active(&self) -> bool {
    self.is_active
  }

  fn handle_events(&mut self, key: KeyEvent) {
    match key.code {
      KeyCode::Char(c) => {
        self.input.push(c);
        self.cursor_index += 1;
      },
      KeyCode::Backspace => {
        let _ = self.input.pop();
        self.cursor_index -= 1;
      },
      // KeyCode::Enter => is_loading = !is_loading,
      _ => ()
    }
  }

  fn render(
    &self,
    frame: &mut Frame,
    area: Rect
  ) {
    let title = Title::from("Tukaj v1.0.0")
      .position(Position::Top)
      .alignment(Alignment::Left);

    let block = Block::new()
      .title(title)
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(self.get_border_color()))
      .padding(Padding::new(
        30,
        30,
        (area.height / 2) - 1,
        0
      ));

    let p = self.get_paragraph()
      .block(block)
      .alignment(Alignment::Center);

    frame.render_widget(p, area);
  }
}

impl TypingWindow {
  #[allow(unused)]
  pub fn config(mut self, config: TypingWindowConfig) -> Self {
    self.config = config;
    self
  }

  pub fn get_paragraph(&self) -> Paragraph {
    let mut lines = Vec::new();

    let remaining_time_line = Line::from(vec![
      Span::from(self.config.time_limit.to_string())
    ]);

    // let info = vec![
    //   Span::styled(content, style)
    // ];

    let info_line = Line::default();

    let text_line = self.generated_text.chars().enumerate().map(|(i, c)| {
      if i == self.cursor_index {
        Span::styled(c.to_string(), Style::default().fg(Color::Black).bg(Color::White))
      } else if i < self.cursor_index {
        if self.input.chars().nth(i) == Some(c) {
          // Span::styled(c.to_string(), Style::default().fg(Color::from_u32(0x805CBF)))
          Span::styled(c.to_string(), Style::default().fg(Color::Rgb(52, 235, 180)))
        } else {
          Span::styled(c.to_string(), Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED))
        }
      } else {
        Span::styled(c.to_string(), Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD))
      }
    }).collect();

    lines.push(remaining_time_line);
    lines.push(text_line);

    let text = Text::from(lines);

    Paragraph::new(text)
  }
}
