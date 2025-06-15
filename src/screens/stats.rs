use std::{cell::RefCell, rc::Rc};

use crate::{
  config::{TukaiConfig, TukaiLayoutColorTypeEnum},
  screens::{Instruction, InstructionWidget, Screen, ToDark},
  storage::{
    stats::Stat,
    storage_handler::{StatOverview, StorageHandler},
  },
};

use ratatui::{
  crossterm::event::KeyEvent,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Style, Stylize},
  symbols,
  text::{Line, Span},
  widgets::{
    Axis, Block, BorderType, Borders, Cell, Chart, Dataset, GraphType, Padding, Paragraph, Row,
    Table,
  },
  Frame,
};

use super::ActiveScreenEnum;

pub struct StatsScreen {
  config: Rc<RefCell<TukaiConfig>>,
}

impl StatsScreen {
  pub fn new(config: Rc<RefCell<TukaiConfig>>) -> Self {
    Self { config }
  }
}

impl Screen for StatsScreen {
  fn increment_time_secs(&mut self) {}

  fn get_config(&self) -> &Rc<RefCell<TukaiConfig>> {
    &self.config
  }

  fn get_screen_name(&self) -> String {
    String::from("Stats")
  }

  fn get_remaining_time(&self) -> usize {
    0
  }

  fn get_next_screen(&self) -> Option<ActiveScreenEnum> {
    None
  }

  fn get_previous_screen(&self) -> Option<ActiveScreenEnum> {
    Some(ActiveScreenEnum::Repeat)
  }

  fn reset(&mut self) {}

  #[allow(dead_code)]
  fn handle_events(&mut self, _key: KeyEvent) -> bool {
    false
  }

  fn render_instructions(&self, frame: &mut Frame, area: Rect) {
    let app_config = self.config.borrow();
    let app_layout = app_config.get_layout();

    let mut instruction_widget = InstructionWidget::new(&app_layout);

    instruction_widget.add_instruction(Instruction::new(
      "Exit",
      "esc",
      TukaiLayoutColorTypeEnum::Secondary,
    ));
    instruction_widget.add_instruction(Instruction::new(
      "Transparent",
      "ctrl-t",
      TukaiLayoutColorTypeEnum::Secondary,
    ));
    instruction_widget.add_instruction(Instruction::new(
      "Typing",
      "ctrl-h",
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

  fn render(&self, frame: &mut Frame, area: Rect) {
    let storage_handler = StorageHandler::new("tukai.bin").init().unwrap();

    let chunks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
      .split(area);

    let left_widget = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
      .split(chunks[0]);

    let right_widget = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![Constraint::Length(7), Constraint::Percentage(100)])
      .split(chunks[1]);

    let last_runs_table_widget_data = storage_handler.get_data_stats_reversed();
    let last_runs_table_widget = self.get_last_runs_table_widget(&last_runs_table_widget_data);

    let chart_widget_data = storage_handler.get_data_for_chart();
    let chart_widget = self.get_chart_widget(&chart_widget_data);

    let best_score_widget = self.get_best_score_widget(&storage_handler);

    let chart_widget_data = storage_handler.get_data_for_overview();
    let stats_overview_widget = self.get_stats_overview_widget(&chart_widget_data);

    frame.render_widget(last_runs_table_widget, left_widget[0]);
    frame.render_widget(chart_widget, left_widget[1]);
    frame.render_widget(stats_overview_widget, right_widget[0]);
    frame.render_widget(best_score_widget, right_widget[1]);
  }

  fn render_popup(&self, _frame: &mut Frame) {}
}

impl StatsScreen {
  /// Returns the right widget (Best score)
  fn get_best_score_widget(&self, storage_handler: &StorageHandler) -> Table {
    let app_config = self.config.borrow();
    let app_layout = &app_config.get_layout();

    let primary_color = app_layout.get_primary_color();
    let text_color = app_layout.get_text_color();

    let stats = storage_handler.get_data_stats_best();

    let block = Block::new()
      .title(" Best score ")
      .title_style(Style::new().fg(primary_color))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(primary_color))
      .border_type(BorderType::Rounded);

    let default_cell_style = Style::default().fg(text_color);

    let rows = stats
      .iter()
      .map(|stat| {
        Row::new(vec![
          Cell::from(stat.get_average_wpm().to_string()).style(default_cell_style),
          Cell::from(format!("{}%", stat.get_accuracy())).style(default_cell_style),
        ])
      })
      .collect::<Vec<Row>>();

    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];

    let default_header_cell_style = Style::default().fg(primary_color).bold();

    let table = Table::new(rows, widths)
      .block(block)
      .column_spacing(1)
      .style(app_config.get_bg_color())
      .header(
        Row::new(vec![
          Cell::from("🔥 Average WPM").style(default_header_cell_style),
          Cell::from("🎯 Accuracy").style(default_header_cell_style),
        ])
        .bottom_margin(1),
      );

    table
  }

  /// Gets the main table widget (Last runs)
  fn get_last_runs_table_widget<'a>(&'a self, stats: &[Stat]) -> Table<'a> {
    let app_config = self.config.borrow();
    let app_layout = app_config.get_layout();

    let primary_color = app_layout.get_primary_color();
    let text_color = app_layout.get_text_color();

    let block = Block::new()
      .title(self.get_title())
      .title_style(Style::new().fg(primary_color))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(primary_color))
      .border_type(BorderType::Rounded);

    let default_cell_style = Style::default().fg(app_layout.get_text_color());

    let rows = stats
      .iter()
      .map(|stat| {
        let duration_pretty = stat.get_duration_pretty();

        Row::new(vec![
          Cell::from(duration_pretty),
          Cell::from(stat.get_average_wpm().to_string()).style(default_cell_style),
          Cell::from(format!("{}%", stat.get_accuracy())).style(default_cell_style),
          Cell::from(stat.get_raw_wpm().to_string())
            .style(Style::default().fg(text_color.to_dark())),
        ])
      })
      .collect::<Vec<Row>>();

    let widths = [
      Constraint::Percentage(25),
      Constraint::Percentage(25),
      Constraint::Percentage(25),
      Constraint::Percentage(25),
    ];

    let default_header_cell_style = Style::default().fg(primary_color).bold();

    let table = Table::new(rows, widths)
      .block(block)
      .column_spacing(1)
      .style(app_config.get_bg_color())
      .highlight_symbol("X")
      .header(
        Row::new(vec![
          Cell::from("⏳ Duration").style(default_header_cell_style),
          Cell::from("🔥 Average WPM").style(default_header_cell_style),
          Cell::from("🎯 Accuracy").style(default_header_cell_style),
          Cell::from("🥩 Raw WPM").style(default_header_cell_style),
        ])
        .bottom_margin(1),
      );

    table
  }

  /// Gets the left bottom widget (Chart)
  fn get_chart_widget<'a>(&self, chart_widget_data: &'a (usize, Vec<(f64, f64)>)) -> Chart<'a> {
    let app_config = self.config.borrow();
    let app_layout = &app_config.get_layout();

    let primary_color = app_layout.get_primary_color();
    let text_color = app_layout.get_text_color();

    let (_best_wpm, chart_data) = chart_widget_data;

    // Validate best_wpm
    // let upper_x_bound = if *best_wpm < 25 { 50 } else { best_wpm + 10 };

    let datasets = vec![Dataset::default()
      .marker(symbols::Marker::Dot)
      .graph_type(GraphType::Scatter)
      .style(Style::default().fg(text_color))
      .data(chart_data)];

    let y_labels = (0..=125)
      .step_by(25)
      .map(|y| Span::from(y.to_string()).style(Style::default().fg(text_color)))
      .collect::<Vec<Span>>();

    let x_axis = Axis::default()
      .style(Style::default().white())
      .bounds([0.0, chart_data.len() as f64]);

    let y_axis = Axis::default()
      .style(Style::default().fg(primary_color))
      .bounds([0.0, 125_f64])
      .labels(y_labels);

    let chart_block = Block::new()
      .title_top(" WPM progress ")
      .title_style(Style::new().fg(primary_color))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(primary_color))
      .border_type(BorderType::Rounded);

    let chart = Chart::new(datasets)
      .block(chart_block)
      .style(app_config.get_bg_color())
      .x_axis(x_axis)
      .y_axis(y_axis);

    chart
  }

  fn get_stats_overview_widget<'a>(&self, stat_overview: &'a StatOverview) -> Paragraph<'a> {
    let app_config = self.config.borrow();
    let app_layout = &app_config.get_layout();

    let primary_color = app_layout.get_primary_color();
    let text_color = app_layout.get_text_color();

    let text = vec![
      Line::default(),
      Line::from(vec![
        Span::from(" Tests count: ").style(Style::default().fg(text_color)),
        Span::from(stat_overview.total_stats_count.to_string())
          .style(Style::default().fg(primary_color).bold()),
      ]),
      Line::from(vec![
        Span::from(" Average WPM: ").style(Style::default().fg(text_color)),
        Span::from(stat_overview.total_average_wpm.to_string())
          .style(Style::default().fg(primary_color).bold()),
      ]),
      Line::from(vec![
        Span::from(" Average accuracy: ").style(Style::default().fg(text_color)),
        Span::from(format!("{}%", stat_overview.total_average_accuracy,))
          .style(Style::default().fg(primary_color).bold()),
      ]),
    ];

    let block = Block::new()
      .title(" Total score ")
      .title_style(Style::new().fg(primary_color))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(primary_color))
      .border_type(BorderType::Rounded);

    let p = Paragraph::new(text)
      .block(block)
      .style(app_config.get_bg_color())
      .alignment(Alignment::Left);

    p
  }
}
