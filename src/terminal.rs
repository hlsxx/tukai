use crossterm::event::{KeyEvent, KeyModifiers};
use crate::tools::loader::Loader;
use crate::windows::path::PathWindow;
use crate::windows::search::SearchWindow;

use crate::traits::Window;

use std::{collections::HashMap, io::{self, Write}, time::Duration};
use ratatui::{
  crossterm::event::{self, KeyCode, KeyEventKind}, layout::{Alignment, Constraint, Direction, Flex, Layout, Rect}, style::{Color, Style, Styled, Stylize}, text::{Line, Span, Text}, widgets::{block::{Position, Title}, Block, Borders, Clear, Paragraph}, DefaultTerminal, Frame
};

#[derive(PartialEq, Hash, Eq)]
enum ActiveWindowEnum {
  Path,
  Search,
  Settings,
  Results
}

pub struct App<'a> {
  is_exit: bool,
  is_loading: bool,
  loader: Loader<'a>,
  active_window: ActiveWindowEnum,
  instructions: HashMap<ActiveWindowEnum, Vec<Span<'a>>>,
  search_window: SearchWindow,
  path_window: PathWindow
}

impl<'a> App<'a> {

  pub fn new() -> Self {
    let mut instructions = HashMap::new();

    let search_instructions = vec![
      Span::styled("[Enter]Start search", Style::default().fg(Color::Green)),
      Span::styled(" [Del]Reset search", Style::default().fg(Color::Red)),
    ];

    instructions.insert(ActiveWindowEnum::Search, search_instructions);

    Self {
      is_loading: false,
      is_exit: false,
      loader: Loader::new(),
      active_window: ActiveWindowEnum::Search,
      instructions,
      search_window: SearchWindow::new(),
      path_window: PathWindow::new()
    }
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !self.is_exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
      std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
  }

  fn draw(&mut self, frame: &mut Frame) {
    let outer_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Percentage(90),
        Constraint::Percentage(10)
      ])
      .split(frame.area());

    let main_layout = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![
        Constraint::Percentage(30),
        Constraint::Percentage(70),
      ])
      .split(outer_layout[0]);

    let left_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Percentage(100)
      ])
      .split(main_layout[0]);

    self.render_results(frame, main_layout[1]);
    self.render_instructions(frame, outer_layout[1]);
    self.render_search(frame, left_layout[0]);
    self.render_path(frame, left_layout[1]);
    self.render_settings(frame, left_layout[2]);

    if self.is_loading {
      self.render_popup(frame);
    }
  }

  fn handle_window_events(&mut self, key: KeyEvent) {
    match self.active_window {
      ActiveWindowEnum::Search => self.search_window.handle_events(key),
      ActiveWindowEnum::Path => self.path_window.handle_events(key),
      // ActiveWindowEnum::Results => self.search.handle_events(key),
      _ => ()
    }
  }

  fn handle_events(&mut self) -> io::Result<()> {
    if crossterm::event::poll(Duration::from_millis(100))? {
      if let event::Event::Key(key) = event::read()? {
        if key.code == KeyCode::Char('1') {
          self.active_window = ActiveWindowEnum::Search;
        } else if key.code == KeyCode::Char('2') {
          self.active_window = ActiveWindowEnum::Path;
        } else if key.code == KeyCode::Char('3') {
          self.active_window = ActiveWindowEnum::Settings;
        } else if key.code == KeyCode::Char('4') {
          self.active_window = ActiveWindowEnum::Results;
        } else if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
          self.is_exit = true;
        } else if key.code == KeyCode::Enter {
          if self.active_window == ActiveWindowEnum::Search {
            if !self.is_loading {
              self.is_loading = true;
            }
          }
        }

        self.handle_window_events(key);
      }
    }

    Ok(())
  }

  fn render_results(&self, frame: &mut Frame, area: Rect) {
    let border_color = self.get_window_border_color(ActiveWindowEnum::Results);

    let block = Block::new()
      .borders(Borders::ALL)
      .border_style(Style::default().fg(border_color))
      .title(Title::from("[4]Results").alignment(Alignment::Center));

    let p = Paragraph::new("Results")
      .block(block);

    frame.render_widget(p, area);
  }

  fn render_instructions(&self, frame: &mut Frame, area: Rect) {
    let default_vec= Vec::new();
    let instructions_spans = self.get_window_instructions().unwrap_or(&default_vec);

    let instructions = Paragraph::new(
      Text::from(Line::from(instructions_spans.clone()))
    ).alignment(Alignment::Center);
    
    frame.render_widget(instructions, area);
  }

  fn render_path(&self, frame: &mut Frame, area: Rect) {
    let border_color = self.get_window_border_color(ActiveWindowEnum::Path);

    let block = Block::new()
      .borders(Borders::ALL)
      .border_style(Style::default().fg(border_color))
      .title(Title::from("[2]Folder").alignment(Alignment::Center));

    let p = Paragraph::new(self.path_window.input.clone())
      .block(block);

    frame.render_widget(p, area);
  }

  fn render_search(&self, frame: &mut Frame, area: Rect) {
    let border_color = self.get_window_border_color(ActiveWindowEnum::Search);

    let block = Block::new()
      .borders(Borders::ALL)
      .border_style(Style::default().fg(border_color))
      .title(Title::from("[1]Search").alignment(Alignment::Center));

    let p = Paragraph::new(self.search_window.input.clone())
      .block(block);

    frame.render_widget(p, area);
  }

  fn render_settings(&self, frame: &mut Frame, area: Rect) {
    let border_color = self.get_window_border_color(ActiveWindowEnum::Settings);

    let block = Block::new()
      .borders(Borders::ALL)
      .border_style(Style::default().fg(border_color))
      .title(Title::from("[3]Settings").alignment(Alignment::Center));

    let p = Paragraph::new("Results")
      .block(block);

    frame.render_widget(p, area);
  }

  fn render_popup(&mut self, frame: &mut Frame) {
    let area = frame.area();

    let block = Block::bordered()
      .border_style(Style::new().fg(Color::Red));

    let text = Text::from(vec![
      Line::from("Loading"),
      Line::from(self.loader.get_slash()),
    ]);

    let p = Paragraph::new(text)
      .block(block)
      .alignment(Alignment::Center)
      .centered()
      .bold();

    let vertical = Layout::vertical([Constraint::Percentage(20)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(20)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    frame.render_widget(Clear, area);
    frame.render_widget(p, area);
  }

  fn get_window_border_color(&self, current_window: ActiveWindowEnum) -> Color {
    if self.active_window == current_window {
      Color::Green
    } else {
      Color::White
    }
  }

  fn get_window_instructions(&self) -> Option<&Vec<Span<'a>>> {
    match self.active_window {
      ActiveWindowEnum::Path => self.instructions.get(&ActiveWindowEnum::Path),
      ActiveWindowEnum::Search => self.instructions.get(&ActiveWindowEnum::Search),
      ActiveWindowEnum::Settings => self.instructions.get(&ActiveWindowEnum::Settings),
      ActiveWindowEnum::Results => self.instructions.get(&ActiveWindowEnum::Results),
    }
  }

}

