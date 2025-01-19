pub mod stats_screen;
pub mod typing_screen;

use std::cell::RefCell;
use std::rc::Rc;

use ratatui::{
  crossterm::event::KeyEvent,
  layout::Rect,
  style::{Color, Style, Stylize},
  text::{Line, Span, Text},
  widgets::Paragraph,
  Frame,
};

use crate::{
  config::TukaiConfig,
  helper::ToDark,
  layout::{Layout as TukaiLayout, LayoutColorTypeEnum},
};

pub struct Instruction<'a> {
  // Instruction title text (description)
  title: &'a str,

  // Instruction shortcut text
  shortcut: &'a str,

  // Layout color
  color_type: LayoutColorTypeEnum,
}

impl<'a> Instruction<'a> {
  pub fn new(title: &'a str, shortcut: &'a str, color_type: LayoutColorTypeEnum) -> Self {
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

  fn get_instruction_color(&self, color_type: &LayoutColorTypeEnum) -> Color {
    match color_type {
      _ => self.layout.get_primary_color()
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
  fn new(config: Rc<RefCell<TukaiConfig>>) -> Self;

  /// Handles key events
  ///
  /// If any key consumed by the screen returns false
  fn handle_events(&mut self, key: KeyEvent) -> bool;

  /// Screen is currently active
  fn is_active(&self) -> bool;
  fn toggle_active(&mut self);

  /// After another screen switched
  fn hide(&mut self) {
    self.toggle_active();
  }

  /// Render screen instructions
  fn render_instructions(&self, frame: &mut Frame, area: Rect);

  /// Render screen
  fn render(&self, frame: &mut Frame, area: Rect);
}
