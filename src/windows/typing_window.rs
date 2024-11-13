use std::{collections::{HashMap, HashSet}, path::Path};

use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
  layout::{Alignment, Constraint, Flex, Layout, Rect}, style::{Color, Modifier, Style, Stylize}, text::{Line, Span, Text}, widgets::{
    block::{Position, Title}, Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap
  }, Frame
};

use crate::common;
use crate::storage::stats::{Stat, TypingDuration};
use crate::storage::storage_handler::StorageHandler;
use crate::configs::typing_window_config::TypingWindowConfig;
use crate::layout::{Layout as TukaiLayout, LayoutColorTypeEnum};
use crate::tools::generator::Generator;
use crate::traits::{ToDark, Window};
use crate::widgets::instructions::{Instruction, InstructionWidget};

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

  /// TODO: User statistics after the run is completed
  pub stats: Stats,

  /// User statistics after the run is completed
  pub stat: Option<Stat>,

  /// The TypingWindow is currently active window
  is_active: bool,

  /// Typing running
  is_running: bool,

  /// Popup is visible
  is_popup_visible: bool,

  pub time_secs: u32,

  /// The current cursor index withing generated_text
  cursor_index: usize,

  /// The TypingWindow custom config
  config: TypingWindowConfig,

  /// Block motto
  motto: String
}

impl Window for TypingWindow {
  fn default() -> Self {
    Self {
      generated_text: Generator::generate_random_string(50),
      input: String::new(),

      stats: Stats::default(),
      stat: None,

      is_active: false,
      is_running: false,
      is_popup_visible: false,

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

  fn hide(&mut self) {
    self.toggle_active();
    self.is_popup_visible = false;
  }

  fn handle_events(&mut self, key: KeyEvent) -> bool {
    if self.cursor_index > 0 && !self.is_running() {
      return false;
    }

    match key.code {
      KeyCode::Esc => {
        if self.is_popup_visible() {
          self.is_popup_visible = false;
          true
        } else {
          false
        }
      },
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
    layout: &TukaiLayout,
    area: Rect
  ) {
    let block = Block::new()
      .title(common::get_title("Stats"))
      .title_bottom(self.motto.as_ref())
      .title_style(Style::default().fg(layout.get_primary_color()))
      .title_alignment(Alignment::Center)
      .style(Style::default().bg(layout.get_background_color()))
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(layout.get_primary_color()))
      .padding(Padding::new(
        40,
        40,
        (area.height / 2) - 5,
        0
      ));

    let p = self.get_paragraph(layout)
      .block(block)
      .alignment(Alignment::Center);

    frame.render_widget(p, area);
  }

  fn render_instructions(
    &self,
    frame: &mut Frame,
    layout: &TukaiLayout,
    area: Rect
  ) {
    let mut instruction_widget = InstructionWidget::new(layout);

    instruction_widget.add_instruction(Instruction::new("Exit", "ESC", LayoutColorTypeEnum::Secondary));
    instruction_widget.add_instruction(Instruction::new("Reset", "CTRL + R", LayoutColorTypeEnum::Secondary));
    instruction_widget.add_instruction(Instruction::new("Layout", "CTRL + S", LayoutColorTypeEnum::Secondary));
    instruction_widget.add_instruction(Instruction::new("Stats", "CTRL + L", LayoutColorTypeEnum::Secondary));

    let block = Block::new()
      .padding(Padding::new(0, 0, area.height / 2, 0));

    let instructions = instruction_widget.get_paragraph()
      .block(block)
      .alignment(Alignment::Center)
      .bg(layout.get_background_color());
    
    frame.render_widget(instructions, area);
  }
}

impl TypingWindow {

  /// If is popup visible
  pub fn is_popup_visible(&self) -> bool {
    self.is_popup_visible
  }

  /// Starts the running typing process
  fn run(&mut self) {
    self.is_running = true;
    self.stat = None;
  }

  /// Stops the running typing process
  pub fn stop(&mut self) {
    self.is_running = false;
    self.is_popup_visible = true;

    if self.stat.is_none() {
      let stat = Stat::new(
        TypingDuration::Minute,
        self.input.len(),
        self.stats.get_mistakes_counter(),
        self.config.time_limit as usize,
      );

      // TODO: Some action if not set into the binary
      StorageHandler::new("test.tukai")
        .init()
        .unwrap()
        .insert_into_stats(&stat);

      self.stat = Some(stat);
    }
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
    if !self.input.pop().is_some() {
      return;
    }

    self.cursor_index -= 1;
    if self.stats.is_char_mistaken(self.cursor_index) {
      self.stats.remove_from_mistakes_indexes(self.cursor_index);
    }
  }

  // Gets the last stat
  pub fn get_last_stat(&self) -> Option<&Stat> {
    if let Some(last_stat) = &self.stat {
      Some(&last_stat)
    } else {
      None
    }
  }

  /// Gets raw wpm
  pub fn get_calculated_raw_wpm(&self) -> usize {
    if let Some(last_stat) = &self.stat {
      last_stat.get_raw_wpm()
    } else {
      0
    }
  }

  /// Gets average WPM
  pub fn get_calculated_wpm(&self) -> usize {
    if let Some(last_stat) = &self.stat {
      last_stat.get_average_wpm()
    } else {
      0
    }
  }

  /// Gets accuracy
  pub fn get_calculated_accuracy(&self) -> f32 {
    if let Some(last_stat) = &self.stat {
      last_stat.get_accuracy()
    } else {
      0.0
    }
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
    self.is_popup_visible = false;
  }

  /// Prepare and get a paragraph
  pub fn get_paragraph(&self, layout: &TukaiLayout) -> Paragraph {
    let mut lines = Vec::new();

    let color = if self.is_active() { layout.get_primary_color() } else { layout.get_primary_color().to_dark() };

    let span = Span::from(
      format!("⏳{}", self.get_remaining_time().to_string()))
      .style(Style::default().fg(color).bold());

    let remaining_time_line = Line::from(vec![span]);

    let text_line = self.generated_text.chars()
      .enumerate()
      .map(|(i, c)| {
        if i == self.cursor_index {
          Span::from(c.to_string())
            .style(Style::default().fg(layout.get_text_current_color()).bg(layout.get_text_current_bg_color()))
        } else if i < self.cursor_index {
          let color = if self.is_active() { layout.get_primary_color() } else { layout.get_primary_color().to_dark() };

          if self.input.chars().nth(i) == Some(c) {
            Span::from(c.to_string())
              .style(Style::default().fg(color))
          } else {
            let color = if self.is_active() { layout.get_error_color() } else { layout.get_error_color().to_dark() };

            Span::from(c.to_string())
              .style(Style::default()
                .fg(color)
                .add_modifier(Modifier::CROSSED_OUT))
          }
        } else {
          let color = if self.is_active() { layout.get_text_color() } else { layout.get_text_color().to_dark() };

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

  pub fn render_popup(
    &self,
    frame: &mut Frame,
    layout: &TukaiLayout
  ) {
    let area = frame.area();

    let block = Block::bordered()
      .style(Style::default().bg(layout.get_background_color()))
      .border_type(BorderType::Rounded)
      .border_style(Style::new().fg(layout.get_primary_color()));

    let text = Text::from(vec![
      Line::from(format!("🔥 Average WPM: {}", self.get_calculated_wpm()))
        .style(Style::default().fg(layout.get_primary_color())),

      Line::from(format!("🎯 Accuracy: {}%", self.get_calculated_accuracy()))
        .style(Style::default().fg(layout.get_primary_color())),

      Line::from(format!("🥩 Raw WPM: {}", self.get_calculated_raw_wpm()))
        .style(Style::default().fg(layout.get_primary_color().to_dark())),

      Line::from(""),
      Line::from(vec![
        Span::from("Try again"),
        Span::from(" <CTRL + R>").style(
          Style::default().fg(layout.get_primary_color()).bold()),
      ]),
    ]);

    let p = Paragraph::new(text)
      .block(block)
      .alignment(Alignment::Center)
      .centered()
      .bold();

    let vertical = Layout::vertical([Constraint::Percentage(22)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(22)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    frame.render_widget(Clear, area);
    frame.render_widget(p, area);
  }

}
