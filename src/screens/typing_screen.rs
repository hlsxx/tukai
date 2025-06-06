use std::{cell::RefCell, collections::HashSet, rc::Rc};

use ratatui::{
  crossterm::event::{KeyCode, KeyEvent},
  layout::{Alignment, Constraint, Flex, Layout, Rect},
  style::{Modifier, Style, Stylize},
  text::{Line, Span, Text},
  widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Wrap},
  Frame,
};

use crate::{
  config::{TukaiConfig, TukaiLayout, TukaiLayoutColorTypeEnum},
  helper::Generator,
  screens::{Instruction, InstructionWidget, Screen, ToDark},
  storage::{stats::Stat, storage_handler::StorageHandler},
};

/// Handler for incorrect symbols
///
/// Inserts incorrect characters into a HashSet
pub struct MistakeHandler {
  mistakes_indexes: HashSet<usize>,
}

impl MistakeHandler {
  /// Creates MistakeHandler with empty HashSet
  fn new() -> Self {
    Self {
      mistakes_indexes: HashSet::new(),
    }
  }

  /// Verifies if the character is mistaken
  pub fn is_char_mistaken(&self, char_index: usize) -> bool {
    self.mistakes_indexes.contains(&char_index)
  }

  /// Adds the typed character into the mistakes
  pub fn add_to_mistakes_indexes(&mut self, char_index: usize) -> bool {
    self.mistakes_indexes.insert(char_index)
  }

  /// Removes the typed character from mistakes
  pub fn remove_from_mistakes_indexes(&mut self, char_index: usize) -> bool {
    self.mistakes_indexes.remove(&char_index)
  }

  /// Returns the current mistake count
  pub fn get_mistakes_counter(&self) -> usize {
    self.mistakes_indexes.len()
  }
}

pub struct TypingScreen {
  /// Application config
  config: Rc<RefCell<TukaiConfig>>,

  /// Random generated text from a words list
  pub generated_text: String,

  /// User typed input
  pub input: String,

  /// Handle incorrect characters
  pub mistake_handler: MistakeHandler,

  /// User statistics after the current run is completed
  pub stat: Option<Stat>,

  /// Typing running
  is_running: bool,

  /// Popup is visible
  is_popup_visible: bool,

  pub time_secs: u32,

  /// The current cursor index withing generated_text
  cursor_index: usize,

  /// Block motto
  motto: String,
}

impl TypingScreen {
  pub fn new(config: Rc<RefCell<TukaiConfig>>) -> Self {
    let generated_text = Generator::generate_random_string(&config.borrow());

    Self {
      config,

      generated_text,

      input: String::new(),

      mistake_handler: MistakeHandler::new(),

      stat: None,

      is_running: false,

      is_popup_visible: false,

      time_secs: 0,

      cursor_index: 0,

      motto: Generator::generate_random_motto(),
    }
  }
}

impl Screen for TypingScreen {
  fn increment_time_secs(&mut self) {
    self.time_secs += 1;
  }

  fn get_config(&self) -> &Rc<RefCell<TukaiConfig>> {
    &self.config
  }

  fn get_remaining_time(&self) -> usize {
    let app_config = &self.config.borrow();

    app_config
      .typing_duration
      .as_seconds()
      .checked_sub(self.time_secs as usize)
      .unwrap_or(0)
  }

  /// Stops the running typing process
  ///
  /// Makes the popup screen visible
  ///
  /// Inserts the created stat into storage
  fn stop(&mut self, storage_handler: &mut StorageHandler) {
    self.is_running = false;
    self.is_popup_visible = true;

    if self.stat.is_none() {
      let stat = Stat::new(
        &self.config.borrow().typing_duration,
        self.input.len(),
        self.mistake_handler.get_mistakes_counter(),
      );

      storage_handler.insert_into_stats(&stat);

      self.stat = Some(stat);
    }
  }

  /// Returns whether typing has begun
  fn is_running(&self) -> bool {
    self.is_running
  }

  fn is_popup_visible(&self) -> bool {
    self.is_popup_visible
  }

  fn get_screen_name(&self) -> String {
    String::from("Typing")
  }

  fn handle_control_events(&mut self, key_event: KeyEvent) -> bool {
    if self.is_popup_visible {
      return false;
    }

    match key_event.code {
      KeyCode::Char('w') | KeyCode::Char('h') => {
      //KeyCode::Char('w') | KeyCode::Backspace => {
        self.delete_last_word();
        true
      }
      _ => false,
    }
  }

  /// Resets all necessary properties
  fn reset(&mut self) {
    self.is_running = false;
    self.time_secs = 0;

    self.mistake_handler = MistakeHandler::new();
    self.cursor_index = 0;
    self.input = String::new();
    self.is_popup_visible = false;

    let app_config = self.config.borrow();
    self.generated_text = Generator::generate_random_string(&app_config);
  }

  fn handle_events(&mut self, key_event: KeyEvent) -> bool {
    if self.cursor_index > 0 && !self.is_running() {
      return false;
    }

    match key_event.code {
      KeyCode::Esc => {
        if self.is_popup_visible() {
          self.is_popup_visible = false;
          true
        } else {
          false
        }
      }
      KeyCode::Char(c) => {
        if self.cursor_index == 0 {
          self.run();
        }

        self.move_cursor_forward_with(c);
        true
      }
      KeyCode::Backspace => {
        self.move_cursor_backward();
        true
      }
      _ => false,
    }
  }

  fn render(&self, frame: &mut Frame, area: Rect) {
    let app_config = self.config.borrow();
    let app_layout = app_config.get_layout();

    let block = Block::new()
      .title(self.get_title())
      .title_alignment(Alignment::Left)
      .title_bottom(self.motto.as_ref())
      .title_style(Style::default().fg(app_layout.get_primary_color()))
      .title_alignment(Alignment::Center)
      .style(app_config.get_bg_color())
      .borders(Borders::ALL)
      .border_type(BorderType::Rounded)
      .border_style(Style::default().fg(app_layout.get_primary_color()))
      .padding(Padding::new(40, 40, (area.height / 2) - 5, 0));

    let p = self
      .get_paragraph(&app_layout)
      .block(block)
      .alignment(Alignment::Left);

    frame.render_widget(p, area);
  }

  fn render_instructions(&self, frame: &mut Frame, area: Rect) {
    let app_config = self.config.borrow_mut();
    let app_layout = app_config.get_layout();

    let mut instruction_widget = InstructionWidget::new(&app_layout);

    instruction_widget.add_instruction(Instruction::new(
      "Exit",
      "esc",
      TukaiLayoutColorTypeEnum::Secondary,
    ));
    instruction_widget.add_instruction(Instruction::new(
      "Reset",
      "ctrl-r",
      TukaiLayoutColorTypeEnum::Secondary,
    ));
    instruction_widget.add_instruction(Instruction::new(
      "Duration",
      "ctrl-d",
      TukaiLayoutColorTypeEnum::Secondary,
    ));
    instruction_widget.add_instruction(Instruction::new(
      "Layout",
      "ctrl-s",
      TukaiLayoutColorTypeEnum::Secondary,
    ));
    instruction_widget.add_instruction(Instruction::new(
      "Transparent",
      "ctrl-t",
      TukaiLayoutColorTypeEnum::Secondary,
    ));
    instruction_widget.add_instruction(Instruction::new(
      "Stats",
      "ctrl-l",
      TukaiLayoutColorTypeEnum::Secondary,
    ));
    instruction_widget.add_instruction(Instruction::new(
      "Language",
      "ctrl-p",
      TukaiLayoutColorTypeEnum::Secondary,
    ));

    let block = Block::new().padding(Padding::new(0, 0, area.height / 2, 0));

    let instructions = instruction_widget
      .get_paragraph()
      .block(block)
      .alignment(Alignment::Center)
      .style(app_config.get_bg_color());

    frame.render_widget(instructions, area);
  }

  /// Renders a popup screen
  ///
  /// Used after the run is completed
  fn render_popup(&self, frame: &mut Frame) {
    let app_config = self.config.borrow();
    let app_layout = app_config.get_layout();
    let area = frame.area();

    let block = Block::bordered()
      .style(app_config.get_bg_color())
      .border_type(BorderType::Rounded)
      .border_style(Style::new().fg(app_layout.get_primary_color()));

    let text = Text::from(vec![
      Line::from(vec![
        Span::from("🔥 Average WPM: "),
        Span::from(format!("{}", self.get_calculated_wpm())).bold(),
      ])
      .style(Style::default().fg(app_layout.get_primary_color())),
      Line::from(vec![
        Span::from("🎯 Accuracy: "),
        Span::from(format!("{}%", self.get_calculated_accuracy())).bold(),
      ])
      .style(Style::default().fg(app_layout.get_primary_color())),
      Line::from(vec![
        Span::from("🥩 Raw WPM: "),
        Span::from(format!("{}", self.get_calculated_raw_wpm())).bold(),
      ])
      .style(Style::default().fg(app_layout.get_primary_color().to_dark())),
      Line::from(""),
      Line::from(vec![
        Span::from("Try again").style(Style::default().fg(app_layout.get_primary_color())),
        Span::from(" ctrl-r").style(Style::default().fg(app_layout.get_primary_color()).bold()),
      ]),
    ]);

    let p = Paragraph::new(text)
      .block(block)
      .alignment(Alignment::Center)
      .centered();

    let vertical = Layout::vertical([Constraint::Percentage(22)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(22)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    frame.render_widget(Clear, area);
    frame.render_widget(p, area);
  }
}

impl TypingScreen {
  /// Starts the running typing process
  ///
  /// Unsets last stat
  fn run(&mut self) {
    self.is_running = true;
    self.stat = None;
  }

  /// Validates an inserted char
  ///
  /// If it is not valid, insert it into the set of mistakes
  fn validate_input_char(&mut self, inserted_char: char) {
    if let Some(generated_char) = self.generated_text.chars().nth(self.cursor_index) {
      if generated_char != inserted_char {
        self
          .mistake_handler
          .add_to_mistakes_indexes(self.cursor_index);
      }
    }
  }

  /// Moves the cursor position forward
  ///
  /// Also validates a char
  fn move_cursor_forward_with(&mut self, c: char) {
    self.validate_input_char(c);
    self.input.push(c);
    self.cursor_index += 1;
  }

  /// Moves the cursor position backward
  ///
  /// Remove the incorrect symbol from the set if its exists
  fn move_cursor_backward(&mut self) {
    if !self.input.pop().is_some() {
      return;
    }

    self.cursor_index -= 1;

    if self.mistake_handler.is_char_mistaken(self.cursor_index) {
      self
        .mistake_handler
        .remove_from_mistakes_indexes(self.cursor_index);
    }
  }

  // Deletes the last word form the input.
  // Handles trailing spaces and updates mistakes.
  pub fn delete_last_word(&mut self) {
    if self.input.is_empty() {
      return;
    }

    let original_input_len = self.input.len();

    // Find the end of the actual contnet (trim trailing whitespace for logic)
    let trimmed_end_len = self.input.trim_end().len();

    if trimmed_end_len == 0 {
      // Input was all spaces
      for i in 0..original_input_len {
        self.mistake_handler.remove_from_mistakes_indexes(i);
      }
      self.input.clear();
      self.cursor_index = 0;
      return;
    }

    // Find the last space before the last word in the trimmed part
    let last_word_start_idx = match self.input[..trimmed_end_len].rfind(' ') {
      Some(space_idx) => space_idx + 1, // Word starts after the space
      None => 0,                        // No space found, word starts at the beginning
    };

    for i in last_word_start_idx..original_input_len {
      self.mistake_handler.remove_from_mistakes_indexes(i);
    }

    self.input.truncate(last_word_start_idx);
    self.cursor_index = self.input.len();
  }

  /// Returns the raw WPM
  pub fn get_calculated_raw_wpm(&self) -> usize {
    if let Some(last_stat) = &self.stat {
      last_stat.get_raw_wpm()
    } else {
      0
    }
  }

  /// Returns the average WPM
  pub fn get_calculated_wpm(&self) -> usize {
    if let Some(last_stat) = &self.stat {
      last_stat.get_average_wpm()
    } else {
      0
    }
  }

  /// Returns the accuracy
  pub fn get_calculated_accuracy(&self) -> f64 {
    if let Some(last_stat) = &self.stat {
      last_stat.get_accuracy()
    } else {
      0.0
    }
  }

  /// Prepares and returns a paragraph.
  ///
  /// If popup window is showed then colors converts to dark.
  pub fn get_paragraph(&self, layout: &TukaiLayout) -> Paragraph {
    let mut lines = Vec::new();

    let (primary_color, error_color, text_color) = {
      let colors = {
        (
          layout.get_primary_color(),
          layout.get_error_color(),
          layout.get_text_color(),
        )
      };

      if self.is_popup_visible() {
        (colors.0.to_dark(), colors.1.to_dark(), colors.2.to_dark())
      } else {
        colors
      }
    };

    let remaining_time_line = Line::from(vec![Span::from(format!(
      "⏳{}",
      self.get_remaining_time(),
    ))
    .style(Style::default().fg(primary_color).bold())]);

    let text_line = self
      .generated_text
      .chars()
      .enumerate()
      .map(|(i, c)| {
        if i == self.cursor_index {
          Span::from(c.to_string()).style(
            Style::default()
              .fg(layout.get_text_current_color())
              .bg(layout.get_text_current_bg_color()),
          )
        } else if i < self.cursor_index {
          if self.input.chars().nth(i) == Some(c) {
            Span::from(c.to_string()).style(Style::default().fg(primary_color))
          } else {
            Span::from(c.to_string()).style(
              Style::default()
                .fg(error_color)
                .add_modifier(Modifier::CROSSED_OUT),
            )
          }
        } else {
          Span::from(c.to_string()).style(Style::default().fg(text_color))
        }
      })
      .collect::<Line>();

    let empty_line = Line::from(Vec::new());

    lines.push(remaining_time_line);

    lines.push(empty_line.clone());

    lines.push(text_line);

    lines.push(empty_line);

    let text = Text::from(lines);

    Paragraph::new(text).wrap(Wrap { trim: true })
  }
}
