use crossterm::event::KeyModifiers;
use ratatui::widgets::{BorderType, Padding};
use crate::config::Package;
use crate::constants::colors;
use crate::event_handler::{EventHandler, TukajEvent};
use crate::helper::get_color_rgb;
use crate::tools::loader::Loader;
use crate::windows::{
  typing_window::{TypingWindow,Stats},
  stats_window::StatsWindow
};

use crate::layout::Layout as TukajLayout;

use crate::traits::Window;

use std::error;
use std::collections::HashMap;
use ratatui::{
  crossterm::event::{KeyCode, KeyEvent},
  layout::{Alignment, Constraint, Flex, Layout, Rect},
  style::{Color, Style, Stylize},
  text::{Line, Span, Text},
  widgets::{Block, Clear, Paragraph},
  DefaultTerminal,
  Frame
};

#[derive(PartialEq, Hash, Eq)]
enum ActiveWindowEnum {
  Typing,
  Stats
}

pub struct App<'a> {
  layout: TukajLayout,

  config_package: Package,

  is_exit: bool,

  is_popup_visible: bool,

  time_secs: u32,

  loader: Loader<'a>,
  active_window: ActiveWindowEnum,
  instructions: HashMap<ActiveWindowEnum, Vec<Span<'a>>>,

  // Windows
  typing_window: TypingWindow,
  stats_window: StatsWindow
}

impl<'a> App<'a> {

  pub fn new(config_package: Package) -> Self {
    let mut instructions = HashMap::new();

    let typing_window_instructions = vec![
      Span::styled("Exit", Style::default().fg(get_color_rgb(colors::SECONDARY))),
      Span::styled("<ESC>", Style::default().fg(get_color_rgb(colors::SECONDARY)).bold()),

      Span::styled(" Reset", Style::default().fg(get_color_rgb(colors::SECONDARY))),
      Span::styled("<CTRL+R>", Style::default().fg(get_color_rgb(colors::SECONDARY)).bold()),

      Span::styled(" Layout", Style::default().fg(get_color_rgb(colors::SECONDARY))),
      Span::styled("<CTRL+I>", Style::default().fg(get_color_rgb(colors::SECONDARY)).bold()),

      Span::styled(" Settings", Style::default().fg(get_color_rgb(colors::SECONDARY))),
      Span::styled(" <CTRL+L>", Style::default().fg(get_color_rgb(colors::SECONDARY)).bold()),
    ];

    let stats_window_instructions = vec![
      Span::styled("<ESC>Exit", Style::default().fg(Color::Yellow)),
      Span::styled(" <Left>Typing", Style::default().fg(Color::Green)),
      Span::styled(" <Right>Stats", Style::default().fg(Color::Red)),
    ];

    instructions.insert(ActiveWindowEnum::Typing, typing_window_instructions);
    instructions.insert(ActiveWindowEnum::Stats, stats_window_instructions);

    Self {
      layout: TukajLayout::default(),

      config_package,

      is_exit: false,
      is_popup_visible: false,

      time_secs: 0,

      loader: Loader::new(),

      active_window: ActiveWindowEnum::Typing,
      instructions,

      typing_window: TypingWindow::default(),
      stats_window: StatsWindow::default()
    }
  }

  pub async fn run(
    &mut self,
    event_handler: &mut EventHandler,
    terminal: &mut DefaultTerminal
  ) -> Result<(), Box<dyn error::Error>> {
    while !self.is_exit {
      match event_handler.next().await? {
        TukajEvent::Key(key_event) => self.handle_events(key_event),
        TukajEvent::Tick => {
          if self.typing_window.is_running() {
            self.time_secs += 1;
          }
        }
      };

      terminal.draw(|frame| self.draw(frame))?;
    }

    Ok(())
  }

  fn draw(
    &mut self,
    frame: &mut Frame
  ) {
    let main_layout = Layout::default()
      .constraints(vec![
        Constraint::Min(0),
        Constraint::Length(3)
      ])
      .split(frame.area());

    match self.active_window {
      ActiveWindowEnum::Typing => {
        if self.is_popup_visible {
          if self.typing_window.is_active() {
            self.typing_window.toggle_active();
          }
        } else if !self.typing_window.is_active() {
          self.typing_window.toggle_active();
        }

        self.typing_window.time_secs = self.time_secs;

        if self.typing_window.get_remaining_time() == 0 {
          self.typing_window.stop();
          self.is_popup_visible = true;
        }

        self.typing_window.render(frame, &self.layout, main_layout[0])
      },
      ActiveWindowEnum::Stats => self.stats_window.render(frame, &self.layout, main_layout[0])
    }

    self.render_instructions(frame, main_layout[1]);

    if self.is_popup_visible {
      self.render_popup(frame);
    }
  }

  fn handle_window_events(&mut self, key: KeyEvent) -> bool {
    let event_occured = match self.active_window {
      ActiveWindowEnum::Typing => self.typing_window.handle_events(key),
      // ActiveWindowEnum::Path => self.path_window.handle_events(key),
      // ActiveWindowEnum::Results => self.search.handle_events(key),
      _ => false
    };

    event_occured
  }

  fn reset(&mut self) {
    self.time_secs = 0;
    self.is_popup_visible = false;
    self.typing_window.stop();
    self.typing_window.reset();
  }

  fn exit(&mut self) {
    self.is_exit = true;
  }

  fn switch_layout(&mut self) {

  }

  /// If the child window does not consume the event, check the keycodes.
  fn handle_events(&mut self, key_event: KeyEvent) {
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
      if key_event.code == KeyCode::Char('r') {
       self.reset();
      } else if key_event.code == KeyCode::Char('i') {
       self.switch_layout();
      }

      return;
    }

    if self.handle_window_events(key_event) {
      return;
    }

    if key_event.code == KeyCode::Esc {
      self.exit();
    } else if key_event.code == KeyCode::Left {
      self.active_window = ActiveWindowEnum::Typing;
    } else if key_event.code == KeyCode::Right {
      self.active_window = ActiveWindowEnum::Stats;
    }
  }

  fn render_instructions(&self, frame: &mut Frame, area: Rect) {
    let default_vec= Vec::new();
    let instructions_spans = self.get_window_instructions().unwrap_or(&default_vec);

    let block = Block::new()
      .padding(Padding::new(0, 0, area.height / 2, 0));

    let instructions = Paragraph::new(Text::from(Line::from(instructions_spans.clone())))
      .block(block)
      .alignment(Alignment::Center)
      .bg(self.layout.get_background_color());
    
    frame.render_widget(instructions, area);
  }

  fn render_popup(&mut self, frame: &mut Frame) {
    let area = frame.area();

    let block = Block::bordered()
      .style(Style::default().bg(self.layout.get_background_color()))
      .border_type(BorderType::Rounded)
      .border_style(Style::new().fg(self.layout.get_primary_color()));

    let text = Text::from(vec![
      Line::from(format!("Average WPM: {}", self.typing_window.get_calculated_wpm()))
        .style(Style::default().fg(self.layout.get_primary_color())),

      Line::from(format!("Raw WPM: {}", self.typing_window.get_calculated_raw_wpm()))
        .style(Style::default().fg(self.layout.get_secondary_color())),

      Line::from(format!("Accuracy: {}%", self.typing_window.get_calculated_accuracy()))
        .style(Style::default().fg(self.layout.get_primary_color())),

      Line::from(""),
      Line::from(vec![
        Span::from("↻ Try again"),
        Span::from(" <CTRL + R>").style(
          Style::default().fg(self.layout.get_primary_color()).bold()),
      ]),
    ]);

    let p = Paragraph::new(text)
      .block(block)
      .alignment(Alignment::Center)
      .centered()
      .bold();

    let vertical = Layout::vertical([Constraint::Percentage(30)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(30)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    frame.render_widget(Clear, area);
    frame.render_widget(p, area);
  }

  fn get_window_instructions(&self) -> Option<&Vec<Span<'a>>> {
    match self.active_window {
      ActiveWindowEnum::Typing => self.instructions.get(&ActiveWindowEnum::Typing),
      ActiveWindowEnum::Stats => self.instructions.get(&ActiveWindowEnum::Stats),
    }
  }

}

