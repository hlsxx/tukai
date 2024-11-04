use std::collections::{HashMap, HashSet};

use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
  layout::{Alignment, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{
    block::{Position, Title},
    Block,
    BorderType,
    Borders,
    Padding,
    Paragraph,
    Wrap
  }, Frame
};

use crate::{
  configs::typing_window_config::TypingWindowConfig,
  constants::{self, colors},
  helper::get_color_rgb,
  tools::generator::Generator,
  traits::Window
};


pub struct Stats {
  mistakes_indexes: HashSet<usize>
}

impl Stats {
  fn default() -> Self {
    Self {
      mistakes_indexes: HashSet::new()
    }
  }

  pub fn is_char_mistaken(&self, char_index: usize) -> bool {
    self.mistakes_indexes.contains(&char_index)
  }

  pub fn remove_from_mistakes_indexes(&mut self, char_index: usize) -> bool {
    self.mistakes_indexes.remove(&char_index)
  }

  pub fn add_to_mistakes_indexes(&mut self, char_index: usize) -> bool {
    self.mistakes_indexes.insert(char_index)
  }

  pub fn get_mistakes_counter(&self) -> usize {
    self.mistakes_indexes.len()
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
  config: TypingWindowConfig,

  motto: String
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

      config: TypingWindowConfig::default(),

      motto: Generator::generate_random_motto()
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
    let title = Title::from("💛 tukaj v1.0.0")
      .position(Position::Top)
      .alignment(Alignment::Left);

    let block = Block::new()
      .title(title)
      .title_bottom(self.motto.as_ref())
      .title_style(Style::default().fg(get_color_rgb(colors::PRIMARY)))
      .title_alignment(Alignment::Center)
      .style(Style::default().bg(get_color_rgb(colors::BACKGROUND)))
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(self.get_border_color()))
      .padding(Padding::new(
        40,
        40,
        (area.height / 2) - 5,
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

  fn validate_input_char(&mut self, c: char) {
    if let Some(generated_char) = self.generated_text.chars().nth(self.cursor_index) {
      if generated_char != c {
        self.stats.add_to_mistakes_indexes(self.cursor_index);
      }
    }
  }

  /// Moves the cursor position forward
  fn move_cursor_forward_with(&mut self, c: char) {
    self.validate_input_char(c);
    self.input.push(c);
    self.cursor_index += 1;
  }

  /// Moves the cursor position backward
  fn move_cursor_backward(&mut self) {
    let _ = self.input.pop();
    self.cursor_index -= 1;

    if self.stats.is_char_mistaken(self.cursor_index) {
      self.stats.remove_from_mistakes_indexes(self.cursor_index);
    }
  }

  /// Calculates raw WPM
  pub fn get_calculated_raw_wpm(&self) -> usize {
    (self.input.len() / 5) * 60 / self.config.time_limit as usize
  }

  /// Calculates WPM
  pub fn get_calculated_wpm(&self) -> usize {
    (self.input.len().saturating_sub(self.stats.get_mistakes_counter()) / 5) * 60 / self.config.time_limit as usize
  }

  /// Calculates accuracy
  pub fn get_calculated_accuracy(&self) -> f32 {
    let accuracy = (self.input.len().saturating_sub(self.stats.get_mistakes_counter()) * 100) as f32 / self.input.len() as f32;
    (accuracy * 100.0).round() / 100.0
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
    self.generated_text = Generator::generate_random_string(
      self.config.time_limit as usize
    );

    self.cursor_index = 0;
    self.input = String::new();
    self.stop();
  }

  /// Prepare and get a paragraph
  pub fn get_paragraph(&self) -> Paragraph {
    let mut lines = Vec::new();

    let remaining_time_line = Line::from(vec![
      Span::from(self.get_remaining_time().to_string())
        .style(
          Style::default()
            .fg(get_color_rgb(colors::PRIMARY))
            .bold()),
    ]);

    let text_line = self.generated_text.chars()
      .enumerate()
      .map(|(i, c)| {
        if i == self.cursor_index {
          Span::from(c.to_string())
            .style(Style::default().fg(Color::Black).bg(Color::White))
        } else if i < self.cursor_index {
          let color = if self.is_active() { colors::PRIMARY } else { colors::SECONDARY };

          if self.input.chars().nth(i) == Some(c) {
            Span::from(c.to_string())
              .style(Style::default().fg(get_color_rgb(color)))
          } else {
            let color = if self.is_active() { colors::ERROR } else { colors::ERROR_SECONDARY };

            Span::from(c.to_string())
              .style(Style::default()
                .fg(get_color_rgb(color))
                .add_modifier(Modifier::CROSSED_OUT))
          }
        } else {
          let color = if self.is_active() { Color::Gray } else { Color::DarkGray };

          Span::from(c.to_string())
            .style(Style::default().fg(color))
        }
      })
      .collect::<Line>();

    lines.push(remaining_time_line);
    lines.push(Line::from(Vec::new()));
    lines.push(text_line);

    let text = Text::from(lines);

    Paragraph::new(text).wrap(Wrap { trim: true } )
  }
}
