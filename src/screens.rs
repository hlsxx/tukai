pub mod stats_screen;
pub mod typing_screen;

use std::cell::RefCell;
use std::rc::Rc;

use ratatui::{
  crossterm::event::KeyEvent,
  layout::Rect,
  style::{Color, Style, Stylize},
  text::{Line, Span, Text},
  widgets::{block::Title, Paragraph},
  Frame,
};

use crate::{
  config::{TukaiConfig, TukaiLayout, TukaiLayoutColorTypeEnum},
  storage::storage_handler::StorageHandler,
};

#[allow(unused)]
pub trait ToDark {
  /// Converts the `(u8, u8, u8)` tuple to a `Color::Rgb`
  ///
  /// # Example
  ///
  /// ```
  /// use ratatui::style::Color
  ///
  /// let rgb: (u8, u8, u8) = (128, 64, 255);
  /// let color = rgb.to_color();
  ///
  /// assert_eq!(color, Color::Rgb(128, 64, 255));
  /// ```
  fn to_dark(self) -> Color;
}

impl ToDark for Color {
  fn to_dark(self) -> Color {
    match self {
      Color::Rgb(r, g, b) => {
        let darkened_r = (r as f32 * (1.0 - 0.2)) as u8;
        let darkened_g = (g as f32 * (1.0 - 0.2)) as u8;
        let darkened_b = (b as f32 * (1.0 - 0.2)) as u8;

        Color::Rgb(darkened_r, darkened_g, darkened_b)
      }
      _ => self,
    }
  }
}

pub struct Instruction<'a> {
  // Instruction title text (description)
  title: &'a str,

  // Instruction shortcut text
  shortcut: &'a str,

  // Layout color
  color_type: TukaiLayoutColorTypeEnum,
}

impl<'a> Instruction<'a> {
  pub fn new(title: &'a str, shortcut: &'a str, color_type: TukaiLayoutColorTypeEnum) -> Self {
    Self {
      title,
      shortcut,
      color_type,
    }
  }
}

pub struct InstructionWidget<'a> {
  layout: &'a TukaiLayout,
  instructions: Vec<Instruction<'a>>,
}

impl<'a> InstructionWidget<'a> {
  pub fn new(layout: &'a TukaiLayout) -> Self {
    Self {
      layout,
      instructions: Vec::new(),
    }
  }

  fn get_instruction_color(&self, color_type: &TukaiLayoutColorTypeEnum) -> Color {
    match color_type {
      _ => self.layout.get_primary_color(),
    }
  }

  pub fn add_instruction(&mut self, instruction: Instruction<'a>) {
    self.instructions.push(instruction);
  }

  /// Returns paragraph contains instructions
  pub fn get_paragraph(&self) -> Paragraph {
    let instructions_spans = self
      .instructions
      .iter()
      .enumerate()
      .flat_map(|(index, instruction)| {
        let color = self.get_instruction_color(&instruction.color_type);

        vec![
          Span::from(format!(" {}", instruction.title)).style(Style::default().fg(color.to_dark())),
          Span::from(format!(
            " {}{}",
            instruction.shortcut,
            if index != self.instructions.len() - 1 {
              " |"
            } else {
              ""
            }
          ))
          .style(Style::default().fg(color).bold()),
        ]
      })
      .collect::<Vec<Span>>();

    Paragraph::new(Text::from(Line::from(instructions_spans)))
  }
}

pub trait Screen {
  // fn new(config: Rc<RefCell<TukaiConfig>>) -> Box<Screen>;
  fn increment_time_secs(&mut self);
  fn get_config(&self) -> &Rc<RefCell<TukaiConfig>>;
  fn get_screen_name(&self) -> String;
  fn get_remaining_time(&self) -> usize;

  fn stop(&mut self, _storage_handler: &mut StorageHandler) {}

  /// Returns the application title
  /// including version from the `Cargo.toml`.
  fn get_title<'a>(&self) -> Title<'a> {
    let app_config = self.get_config().borrow();
    let app_layout = app_config.get_layout();

    Title::from(format!(
      " tukai v{} 》{} 》{} ",
      env!("CARGO_PKG_VERSION"),
      app_layout.get_active_layout_name(),
      self.get_screen_name()
    ))
  }

  fn reset(&mut self);

  /// Handles key events
  ///
  /// If any key consumed by the screen returns false
  fn handle_events(&mut self, key: KeyEvent) -> bool;

  /// Typing is running
  fn is_running(&self) -> bool {
    false
  }

  /// Returns whether the popup is visible.
  ///
  /// Default set to false (not used in stats screen)
  fn is_popup_visible(&self) -> bool {
    false
  }

  /// Renders screen instructions.
  ///
  /// Visible of the bottom of screen
  fn render_instructions(&self, frame: &mut Frame, area: Rect);

  /// Renders screen widgets.
  fn render(&self, frame: &mut Frame, area: Rect);

  /// Renders a popup screen
  ///
  /// Used after the run is completed
  fn render_popup(&self, frame: &mut Frame);

  /// Handles control-modified key events specific to the screen.
  ///
  /// True if event consumed, false otherwise.
  #[allow(unused_variables)]
  fn handle_control_events(&mut self, key_event: KeyEvent) -> bool {
    false
  }
}
