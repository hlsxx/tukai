use crate::{common, layout::LayoutColorTypeEnum, storage::storage_handler::StorageHandler, traits::Window, widgets::instructions::{Instruction, InstructionWidget}};

use crossterm::event::KeyEvent;

use ratatui::{
  layout::{Alignment, Constraint, Rect},
  style::{Style, Stylize},
  widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table},
  Frame
};

use crate::layout::Layout as TukaiLayout;
use crate::traits::ToDark;

pub struct StatsWindow {
  is_active: bool
}

impl Window for StatsWindow {
  fn default() -> Self {
    Self {
      is_active: false
    }
  }

  fn toggle_active(&mut self) {
    self.is_active = !self.is_active;
  }

  fn is_active(&self) -> bool {
    self.is_active
  }

  #[allow(dead_code)]
  fn handle_events(&mut self, _key: KeyEvent) -> bool {
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
    instruction_widget.add_instruction(Instruction::new("Typing", "CTRL + J", LayoutColorTypeEnum::Secondary));

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
    let storage_handler = StorageHandler::new("tukai.bin")
      .init()
      .unwrap();
    
    let stats = storage_handler.get_data_stats_reversed().unwrap();

    let block = Block::new()
      .title(common::get_title(layout.get_active_layout_title(), "Stats"))
      .title_style(Style::new().fg(layout.get_primary_color()))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(layout.get_primary_color()))
      .border_type(BorderType::Rounded);

    let default_cell_style = Style::default()
      .fg(layout.get_text_color());

    let rows = stats.iter()
      .map(|stat| {
        Row::new(vec![
          Cell::from(stat.get_duration().to_string())
            .style(Style::default().fg(layout.get_text_color().to_dark())),

          Cell::from(stat.get_average_wpm().to_string())
            .style(default_cell_style),

          Cell::from(format!("{}%", stat.get_accuracy().to_string()))
            .style(default_cell_style),

          Cell::from(stat.get_raw_wpm().to_string())
            .style(Style::default().fg(layout.get_text_color().to_dark()))
        ])
      }).collect::<Vec<Row>>();

    let widths = [
      Constraint::Percentage(25),
      Constraint::Percentage(25),
      Constraint::Percentage(25),
      Constraint::Percentage(25)
    ];

    let default_header_cell_style = Style::default()
      .fg(layout.get_primary_color())
      .bold();

    let table = Table::new(rows, widths)
      .block(block)
      .column_spacing(1)
      .style(Style::new().bg(layout.get_background_color()))
      .header(
        Row::new(vec![
          Cell::from("⏳ Duration")
            .style(default_header_cell_style),

          Cell::from("🔥 Average WPM")
            .style(default_header_cell_style),

          Cell::from("🎯 Accuracy")
            .style(default_header_cell_style),

          Cell::from("🥩 Raw WPM")
            .style(default_header_cell_style),
        ]).bottom_margin(1)
      );

    frame.render_widget(table, area);
  }

}
