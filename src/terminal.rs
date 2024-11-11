use crossterm::event::KeyModifiers;
use ratatui::widgets::BorderType;
use crate::config::Package;
use crate::event_handler::{EventHandler, TukaiEvent};
use crate::tools::loader::Loader;
use crate::windows::{
  typing_window::{TypingWindow,Stats},
  stats_window::StatsWindow
};

use crate::layout::Layout as TukaiLayout;

use crate::traits::Window;

use std::error;
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
  layout: TukaiLayout,

  config_package: Package,

  is_exit: bool,

  is_popup_visible: bool,

  time_secs: u32,

  loader: Loader<'a>,
  active_window: ActiveWindowEnum,

  // Windows
  typing_window: TypingWindow,
  stats_window: StatsWindow
}

impl<'a> App<'a> {

  pub fn new(config_package: Package) -> Self {
    Self {
      layout: TukaiLayout::default(),

      config_package,

      is_exit: false,
      is_popup_visible: false,

      time_secs: 0,

      loader: Loader::new(),

      active_window: ActiveWindowEnum::Typing,

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
        TukaiEvent::Key(key_event) => self.handle_events(key_event),
        TukaiEvent::Tick => {
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
          self.typing_window.stop(true);
          self.is_popup_visible = true;
        }

        self.typing_window.render(frame, &self.layout, main_layout[0]);
        self.typing_window.render_instructions(frame, &self.layout, main_layout[1]);
      },
      ActiveWindowEnum::Stats => {
        self.stats_window.render(frame, &self.layout, main_layout[0]);
        self.stats_window.render_instructions(frame, &self.layout, main_layout[1]);
      }
    }

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
    self.typing_window.reset();
  }

  fn exit(&mut self) {
    self.is_exit = true;
  }

  /// If the child window does not consume the event, check the keycodes.
  fn handle_events(&mut self, key_event: KeyEvent) {
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
      match key_event.code {
        KeyCode::Char(c) => {
          match c {
            'r' => self.reset(),
            's' => self.layout.switch_active_layout(),
            'l' => self.active_window = ActiveWindowEnum::Stats,
            'c' => self.exit(),
            _ => {}
          }
        },
        _ => {}
      }

      return;
    }

    if self.handle_window_events(key_event) {
      return;
    }

    if key_event.code == KeyCode::Esc {
      if self.is_popup_visible {
        self.reset();
      } else {
        self.exit();
      }
    } else if key_event.code == KeyCode::Left {
      self.active_window = ActiveWindowEnum::Typing;
    } else if key_event.code == KeyCode::Right {
      self.active_window = ActiveWindowEnum::Stats;
    }
  }

  fn render_popup(&mut self, frame: &mut Frame) {
    let area = frame.area(); let block = Block::bordered()
      .style(Style::default().bg(self.layout.get_background_color()))
      .border_type(BorderType::Rounded)
      .border_style(Style::new().fg(self.layout.get_primary_color()));

    let text = Text::from(vec![
      Line::from(format!("Average WPM: {}", self.typing_window.get_calculated_wpm()))
        .style(Style::default().fg(self.layout.get_primary_color())),

      Line::from(format!("Accuracy: {}%", self.typing_window.get_calculated_accuracy()))
        .style(Style::default().fg(self.layout.get_primary_color())),

      Line::from(format!("Raw WPM: {}", self.typing_window.get_calculated_raw_wpm()))
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

}

