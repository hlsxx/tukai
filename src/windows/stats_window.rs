use crate::{
  configs::app_config::AppConfig, helper::get_title, layout::LayoutColorTypeEnum, storage::{stats::Stat, storage_handler::{StatOverview, StorageHandler}}, windows::{Instruction, InstructionWidget, Window}
};

use ratatui::{
  crossterm::event::KeyEvent, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, symbols, text::{Line, Span}, widgets::{Axis, Block, BorderType, Borders, Cell, Chart, Dataset, GraphType, Padding, Paragraph, Row, Table}, Frame
};

use crate::helper::ToDark;

pub struct StatsWindow {
  is_active: bool
}

impl Window for StatsWindow {
  fn default() -> Self {
    Self {
      is_active: false
    }
  }

  fn toggle_active(&mut self) { self.is_active = !self.is_active; }

  fn is_active(&self) -> bool {
    self.is_active
  }

  #[allow(dead_code)]
  fn handle_events(&mut self, _key: KeyEvent) -> bool {
    false
  }

  fn render_instructions(
    &self,
    frame: &mut Frame,
    app_config: &AppConfig,
    area: Rect
  ) {
    let app_layout = app_config.get_layout();
    let mut instruction_widget = InstructionWidget::new(&app_layout);

    instruction_widget.add_instruction(Instruction::new("Exit", "esc", LayoutColorTypeEnum::Secondary));
    instruction_widget.add_instruction(Instruction::new("Transparent", "ctrl + t", LayoutColorTypeEnum::Secondary));
    instruction_widget.add_instruction(Instruction::new("Typing", "ctrl + h", LayoutColorTypeEnum::Secondary));

    let block = Block::new()
      .padding(Padding::new(0, 0, area.height / 2, 0));

    let instructions = instruction_widget.get_paragraph()
      .block(block)
      .alignment(Alignment::Center)
      .style(app_config.get_bg_color());

    frame.render_widget(instructions, area);
  }

  fn render(
    &self,
    frame: &mut Frame,
    app_config: &AppConfig,
    version: &String,
    area: Rect
  ) {
    let storage_handler = StorageHandler::new("tukai.bin")
      .init()
      .unwrap();

    let chunks = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![
        Constraint::Percentage(70),
        Constraint::Percentage(30)
      ])
      .split(area);

    let left_widget = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Percentage(50),
        Constraint::Percentage(50)
      ])
      .split(chunks[0]);

    let right_widget = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Length(5),
        Constraint::Percentage(100)
      ])
      .split(chunks[1]);
    
    let last_runs_table_widget_data = storage_handler.get_data_stats_reversed().unwrap();
    let last_runs_table_widget = self.get_last_runs_table_widget(
      &app_config,
      &last_runs_table_widget_data,
      version);

    let chart_widget_data = storage_handler.get_data_for_chart();
    let chart_widget = self.get_chart_widget(&app_config, &chart_widget_data);

    let best_score_widget = self.get_best_score_widget(&app_config, &storage_handler);

    let chart_widget_data = storage_handler.get_data_for_overview();
    let stats_overview_widget = self.get_stats_overview_widget(&app_config, &chart_widget_data);

    frame.render_widget(last_runs_table_widget, left_widget[0]);
    frame.render_widget(chart_widget, left_widget[1]);
    frame.render_widget(stats_overview_widget, right_widget[0]);
    frame.render_widget(best_score_widget, right_widget[1]);
  }
}

impl StatsWindow {

  /// Returns the right widget (Best score)
  fn get_best_score_widget(
    &self,
    app_config: &AppConfig,
    storage_handler: &StorageHandler
  ) -> Table {
    let app_layout = &app_config.get_layout();

    let stats = storage_handler.get_data_stats_bets().unwrap();

    let block = Block::new()
      .title(" Best score ")
      .title_style(Style::new().fg(app_layout.get_primary_color()))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(app_layout.get_primary_color()))
      .border_type(BorderType::Rounded);

    let default_cell_style = Style::default()
      .fg(app_layout.get_text_color());

    let rows = stats.iter()
      .map(|stat| {
        Row::new(vec![
          Cell::from(stat.get_average_wpm().to_string())
            .style(default_cell_style),

          Cell::from(format!("{}%", stat.get_accuracy().to_string()))
            .style(default_cell_style),
        ])
      }).collect::<Vec<Row>>();

    let widths = [
      Constraint::Percentage(50),
      Constraint::Percentage(50),
    ];

    let default_header_cell_style = Style::default()
      .fg(app_layout.get_primary_color())
      .bold();

    let table = Table::new(rows, widths)
      .block(block)
      .column_spacing(1)
      .style(app_config.get_bg_color())
      .header(
        Row::new(vec![
          Cell::from("🔥 Average WPM")
            .style(default_header_cell_style),

          Cell::from("🎯 Accuracy")
            .style(default_header_cell_style),
        ]).bottom_margin(1)
      );

    table
  }

  /// Gets the main table widget (Last runs)
  fn get_last_runs_table_widget<'a>(
    &self,
    app_config: &AppConfig,
    stats: &Vec<Stat>,
    version: &String
  ) -> Table<'a> {
    let app_layout = &app_config.get_layout();

    let block_title = get_title(
      version,
      app_layout.get_active_layout_name(),
      "Stats"
    );

    let block = Block::new()
      .title(block_title)
      .title_style(Style::new().fg(app_layout.get_primary_color()))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(app_layout.get_primary_color()))
      .border_type(BorderType::Rounded);

    let default_cell_style = Style::default()
      .fg(app_layout.get_text_color());

    let rows = stats.iter()
      .map(|stat| {
        Row::new(vec![
          Cell::from(stat.get_duration().to_string())
            .style(Style::default().fg(app_layout.get_text_color().to_dark())),

          Cell::from(stat.get_average_wpm().to_string())
            .style(default_cell_style),

          Cell::from(format!("{}%", stat.get_accuracy().to_string()))
            .style(default_cell_style),

          Cell::from(stat.get_raw_wpm().to_string())
            .style(Style::default().fg(app_layout.get_text_color().to_dark()))
        ])
      }).collect::<Vec<Row>>();

    let widths = [
      Constraint::Percentage(25),
      Constraint::Percentage(25),
      Constraint::Percentage(25),
      Constraint::Percentage(25)
    ];

    let default_header_cell_style = Style::default()
      .fg(app_layout.get_primary_color())
      .bold();

    let table = Table::new(rows, widths)
      .block(block)
      .column_spacing(1)
      .style(app_config.get_bg_color())
      .highlight_symbol("X")
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

    table
  }

  /// Gets the left bottom widget (Chart)
  fn get_chart_widget<'a>(
    &self,
    app_config: &AppConfig,
    chart_widget_data: &'a (usize, Vec<(f64, f64)>)
  ) -> Chart<'a> {
    let app_layout = &app_config.get_layout();
    let (_best_wpm, chart_data) = chart_widget_data;

    // Validate best_wpm
    // let upper_x_bound = if *best_wpm < 25 { 50 } else { best_wpm + 10 };

    let datasets = vec![
      Dataset::default()
        .marker(symbols::Marker::Dot)
        .graph_type(GraphType::Scatter)
        .style(Style::default().fg(app_layout.get_text_color()))
        .data(&chart_data)
    ];

    let x_axis = Axis::default()
      .style(Style::default().white())
      .bounds([0.0, chart_data.len() as f64]);

    let y_axis = Axis::default()
      .style(Style::default().fg(app_layout.get_text_color()))
      .bounds([0.0, 125 as f64])
      .labels((0..=125).step_by(25).map(|y| y.to_string()).collect::<Vec<String>>());

    let chart_block = Block::new()
      .title_top(" WPM progress ")
      .title_style(Style::new().fg(app_layout.get_primary_color()))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(app_layout.get_primary_color()))
      .border_type(BorderType::Rounded);

    let chart = Chart::new(datasets)
      .block(chart_block)
      .style(app_config.get_bg_color())
      .x_axis(x_axis)
      .y_axis(y_axis);

    chart
  }

  fn get_stats_overview_widget<'a>(
    &self,
    app_config: &AppConfig,
    stat_overview: &'a StatOverview,
  ) -> Paragraph<'a> {
    let app_layout = &app_config.get_layout();

    let text = vec![
      Line::default(),

      Line::from(vec![
        Span::from(" Total average WPM: ").style(Style::default().fg(app_layout.get_text_color())),
        Span::from(stat_overview.total_average_wpm.to_string())
          .style(Style::default().fg(app_layout.get_primary_color()).bold())
      ]),

      Line::from(vec![
        Span::from(" Total average accuracy: ").style(Style::default().fg(app_layout.get_text_color())),
        Span::from(format!("{}%", stat_overview.total_average_accuracy.to_string()))
          .style(Style::default().fg(app_layout.get_primary_color()).bold())
      ]),
    ];

    let block = Block::new()
      .title(" Total score ")
      .title_style(Style::new().fg(app_layout.get_primary_color()))
      .borders(Borders::ALL)
      .border_style(Style::default().fg(app_layout.get_primary_color()))
      .border_type(BorderType::Rounded);

    let p = Paragraph::new(text)
      .block(block)
      .style(app_config.get_bg_color())
      .alignment(Alignment::Left);

    p
  }
}
