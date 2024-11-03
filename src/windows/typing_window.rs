use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
  Frame,
  layout::{Alignment, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span, Text},
  widgets::{
    block::{Position, Title},
    Block,
    BorderType,
    Borders,
    Padding,
    Paragraph,
    Wrap
  }
};

use crate::{
  configs::typing_window_config::TypingWindowConfig,
  constants::{self, colors},
  helper::get_color_rgb,
  tools::generator::Generator,
  traits::Window
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
  /// Random generated text
  pub generated_text: String,

  /// User typed input
  pub input: String,

  /// User statistics after the run is completed
  pub stats: Stats,

  /// The TypingWindow is currently active window
  is_active: bool,

  /// Typing running
  is_running: bool,

  pub time_secs: u32,

  /// The current cursor index withing generated_text
  cursor_index: usize,

  /// The TypingWindow custom config
  config: TypingWindowConfig
}

impl Window for TypingWindow {
  fn default() -> Self {
    Self {
      generated_text: Generator::generate_random_string(50),
      input: String::new(),

      stats: Stats::default(),

      is_active: false,
      is_running: false,

      time_secs: 0,

      cursor_index: 0,

      config: TypingWindowConfig::default()
    }
  }

  fn toggle_active(&mut self) {
    self.is_active = !self.is_active;
  }

  fn is_active(&self) -> bool {
    self.is_active
  }

  fn handle_events(&mut self, key: KeyEvent) -> bool {
    if self.cursor_index > 0 && !self.is_running() {
      return false;
    }

    match key.code {
      KeyCode::Char(c) => {
        if self.cursor_index == 0 {
          self.run();
        }

        self.move_cursor_forward_with(c);
        true
      },
      KeyCode::Backspace => {
        self.move_cursor_backward();
        true
      },
      _ => false
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
      .style(Style::default().bg(get_color_rgb(colors::BACKGROUND)))
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

  /// Starts the running typing process
  fn run(&mut self) {
    self.is_running = true;
  }

  /// Stops the running typing process
  pub fn stop(&mut self) {
    self.is_running = false;
  }

  /// Moves the cursor position forward
  fn move_cursor_forward_with(&mut self, c: char) {
    self.input.push(c);
    self.cursor_index += 1;
  }

  /// Moves the cursor position backward
  fn move_cursor_backward(&mut self) {
    let _ = self.input.pop();
    self.cursor_index -= 1;
  }

  #[allow(unused)]
  pub fn config(mut self, config: TypingWindowConfig) -> Self {
    self.config = config;
    self
  }

  /// Calculate the remaining time
  pub fn get_remaining_time(&self) -> u32 {
    self.config.time_limit.checked_sub(self.time_secs).unwrap_or(0)
  }

  /// Returns if typing already began
  pub fn is_running(&self) -> bool {
     self.is_running
  }

  /// Reset typing window
  pub fn reset(&mut self) {
    self.generated_text = Generator::generate_random_string(50);
    self.cursor_index = 0;
    self.input = String::new();
  }

  /// Replace space with a dot char
  fn get_formatted_char(&self, c: char) -> String {
    if c == ' ' {
      '•'.to_string()
    } else {
      c.to_string()
    }
  }

  /// Prepare and get a paragraph
  pub fn get_paragraph(&self) -> Paragraph {
    let mut lines = Vec::new();

    let remaining_time_line = Line::from(vec![
      // Span::from(self.config.time_limit.to_string())
      Span::from(self.get_remaining_time().to_string())
    ]);

    // let current_time_lock = self.current_time.lock().await;

    let info_line = Line::from(vec![
      Span::from("125".to_string()).style(Style::default()),
      Span::from(" WPM").style(Style::default())
    ]);

    let text_line = self.generated_text.chars()
      .enumerate()
      .map(|(i, c)| {
        if i == self.cursor_index {
          Span::from(self.get_formatted_char(c))
            .style(Style::default().fg(Color::Black).bg(Color::White))
        } else if i < self.cursor_index {
          if self.input.chars().nth(i) == Some(c) {
            Span::from(self.get_formatted_char(c))
              .style(Style::default().fg(get_color_rgb(colors::PRIMARY)))
          } else {
            Span::from(self.get_formatted_char(c))
              .style(Style::default().fg(get_color_rgb(colors::ERROR)).add_modifier(Modifier::UNDERLINED))
          }
        } else {
          Span::from(c.to_string())
            .style(Style::default().fg(Color::Gray))
        }
      })
      .collect::<Line>();

    lines.push(remaining_time_line);
    // lines.push(info_line);
    lines.push(text_line);

    let text = Text::from(lines);

    Paragraph::new(text).wrap(Wrap { trim: true } )
  }
}
