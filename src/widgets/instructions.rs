use ratatui::{style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::Paragraph};

use crate::layout::{LayoutColorTypeEnum, Layout as TukaiLayout};

pub struct Instruction<'a> {
  title: &'a str,
  shortcut: &'a str,
  color_type: LayoutColorTypeEnum
}

impl<'a> Instruction<'a> {

  pub fn new(
    title: &'a str,
    shortcut: &'a str,
    color_type: LayoutColorTypeEnum
  ) -> Self {
    Self {
      title,
      shortcut,
      color_type
    }
  }

}

pub struct InstructionWidget<'a> {
  layout: &'a TukaiLayout,
  instructions: Vec<Instruction<'a>>
}

impl<'a> InstructionWidget<'a> {
  pub fn new(layout: &'a TukaiLayout) -> Self {
    Self {
      layout,
      instructions: Vec::new()
    }
  }

  fn get_instruction_color(&self, color_type: &LayoutColorTypeEnum) -> Color {
    match color_type {
      LayoutColorTypeEnum::Primary => self.layout.get_primary_color(),
      LayoutColorTypeEnum::Secondary => self.layout.get_secondary_color(),
      LayoutColorTypeEnum::Text => self.layout.get_text_color(),
      LayoutColorTypeEnum::TextReverse => self.layout.get_text_reverse_color(),
      LayoutColorTypeEnum::Error => self.layout.get_error_color(),
      LayoutColorTypeEnum::Background => self.layout.get_background_color()
    }
  }

  pub fn add_instruction(&mut self, instruction: Instruction<'a>) {
    self.instructions.push(instruction);
  }

  pub fn get_paragraph(&self) -> Paragraph {
    let instructions_spans = self.instructions.iter().flat_map(|instruction| {
      let color = self.get_instruction_color(&instruction.color_type);

      vec![
        Span::from(format!(" {}", instruction.title)).style(Style::default().fg(color)),
        Span::from(format!("<{}>", instruction.shortcut)).style(Style::default().fg(color).bold()),
      ]
    }).collect::<Vec<Span>>();

    Paragraph::new(Text::from(Line::from(instructions_spans)))
  }

}


