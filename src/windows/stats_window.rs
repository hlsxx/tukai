use crate::{common, layout::LayoutColorTypeEnum, storage::storage_handler::{self, StorageHandler}, traits::Window, widgets::instructions::{Instruction, InstructionWidget}};

use crossterm::event::{KeyCode, KeyEvent};
use std::env;

use ratatui::{
  layout::{Alignment, Constraint, Rect},
  style::{Style, Stylize},
  widgets::{block::Title, Block, BorderType, Borders, Padding, Paragraph, Row, Table},
  Frame
};

use crate::layout::Layout as TukaiLayout;

pub struct StatsWindow {
  pub input: String,

  is_active: bool
}

impl Window for StatsWindow {
  fn default() -> Self {
    let path = env::current_dir().expect("Error getting current path");

    Self {
      input: path.to_string_lossy().into_owned(),

      is_active: false
    }
  }

  fn toggle_active(&mut self) {
    self.is_active = !self.is_active;
  }

  fn is_active(&self) -> bool {
    self.is_active
  }

  fn handle_events(&mut self, key: KeyEvent) -> bool {
    false
    // match key.code {
    //   KeyCode::Char(c) => self.input.push(c),
    //   KeyCode::Backspace => { let _ = self.input.pop(); },
    //   _ => ()
    // }
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
    instruction_widget.add_instruction(Instruction::new("Layout", "CTRL + I", LayoutColorTypeEnum::Secondary));
    instruction_widget.add_instruction(Instruction::new("Settings", "CTRL + L", LayoutColorTypeEnum::Secondary));

    let block = Block::new()
      .padding(Padding::new(0, 0, area.height / 2, 0));

    let instructions = instruction_widget.get_paragraph()
      .block(block)
      .alignment(Alignment::Center)
      .bg(layout.get_background_color());
    
    frame.render_widget(instructions, area);
  }

  fn render(
    &self,
    frame: &mut Frame,
    layout: &TukaiLayout,
    area: Rect
  ) {
    let storage_handler = StorageHandler::new("test.tukai")
      .init();

    let stats = storage_handler.get_data_stats().unwrap();

    let block = Block::new()
      .title(common::get_title())
      .borders(Borders::ALL)
      .border_style(Style::default().fg(layout.get_secondary_color()))
      .border_type(BorderType::Rounded);

    let rows = stats.iter()
      .map(|stat| {
        Row::new(vec![
          stat.get_average_wpm().to_string(),
          stat.get_raw_wpm().to_string(),
          format!("{}%", stat.get_accuracy().to_string())
        ]).style(Style::default().fg(layout.get_primary_color()))
      }).collect::<Vec<Row>>();

    let widths = [
      Constraint::Percentage(33),
      Constraint::Percentage(33),
      Constraint::Percentage(33),
    ];

    let table = Table::new(rows, widths)
      .block(block)
      .column_spacing(1)
      .style(Style::new().blue())
      .header(
        Row::new(vec!["Average WPM", "Raw WPM", "Accuracy"])
          .style(Style::new().bold().fg(layout.get_secondary_color()))
          .bottom_margin(1),
      );

    frame.render_widget(table, area);
  }

}
