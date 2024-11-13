use crossterm::event::KeyModifiers;
use crate::config::Package;
use crate::event_handler::{EventHandler, TukaiEvent};
use crate::storage::storage_handler::StorageHandler;
use crate::tools::loader::Loader;
use crate::windows::{
  typing_window::TypingWindow,
  stats_window::StatsWindow
};

use crate::layout::Layout as TukaiLayout;

use crate::traits::Window;

use std::error;
use std::path::{Path, PathBuf};
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

pub struct AppConfig {
  file_path: PathBuf,
  layout: TukaiLayout
}

impl AppConfig {
  pub fn default() -> Self {
    Self {
      file_path: PathBuf::from("tukai.bin"),
      layout: TukaiLayout::default()
    }
  }
}

pub struct AppConfigBuilder {
  file_path: Option<PathBuf>,
  layout: Option<TukaiLayout>
}

impl AppConfigBuilder {
  pub fn new() -> Self {
    Self {
      file_path: None,
      layout: None
    }
  }

  pub fn file_path<P: AsRef<Path>>(mut self, file_path: P) -> Self {
    self.file_path = Some(file_path.as_ref().to_path_buf());
    self
  }

  pub fn layout(mut self, layout: TukaiLayout) -> Self {
    self.layout = Some(layout);
    self
  }

  pub fn build(self) -> AppConfig {
    let config_default = AppConfig::default();

    AppConfig {
      file_path: self.file_path.unwrap_or(config_default.file_path),
      layout: self.layout.unwrap_or(config_default.layout)
    }
  }
}

pub struct App<'a> {
  config: AppConfig,

  storage_handler: Option<StorageHandler>,

  is_exit: bool,

  time_secs: u32,

  loader: Loader<'a>,
  active_window: ActiveWindowEnum,

  // Windows
  typing_window: TypingWindow,
  stats_window: StatsWindow
}

impl<'a> App<'a> {

  /// Creates new Tukai App
  pub fn new(config: AppConfig) -> Self {
    Self {
      config,

      storage_handler: None,

      is_exit: false,

      time_secs: 0,

      loader: Loader::new(),

      active_window: ActiveWindowEnum::Typing,

      typing_window: TypingWindow::default(),
      stats_window: StatsWindow::default()
    }
  }

  fn get_config_layout(&self) -> &TukaiLayout {
    &self.config.layout
  }

  /// Inits the App
  ///
  /// Storage handler (not reuired)
  pub fn init(mut self) -> Self {
    match StorageHandler::new(&self.config.file_path).init() {
      Ok(storage_handler) => {
        if let Some(active_layout_name) = storage_handler.get_active_layout_name() {
          self.config.layout.active_layout_name(active_layout_name);
        }

        self.storage_handler = Some(storage_handler);
      },
      Err(_) => {}
    }

    self
  }

  /// Runs the Tukai application
  /// 
  /// Renders TUI
  ///
  /// Handle events
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
        if self.typing_window.is_popup_visible() {
          if self.typing_window.is_active() {
            self.typing_window.toggle_active();
          }
        } else if !self.typing_window.is_active() {
          self.typing_window.toggle_active();
        }

        self.typing_window.time_secs = self.time_secs;

        if self.typing_window.get_remaining_time() == 0 {
          self.typing_window.stop(self.storage_handler.as_mut());
        }

        // Renders
        self.typing_window.render(frame, &self.get_config_layout(), main_layout[0]);
        self.typing_window.render_instructions(frame, &self.get_config_layout(), main_layout[1]);

        if self.typing_window.is_popup_visible() {
          self.typing_window.render_popup(frame, &self.get_config_layout());
        }
      },
      ActiveWindowEnum::Stats => {
        self.stats_window.render(frame, &self.get_config_layout(), main_layout[0]);
        self.stats_window.render_instructions(frame, &self.get_config_layout(), main_layout[1]);
      }
    }
  }

  fn handle_window_events(&mut self, key: KeyEvent) -> bool {
    let event_occured = match self.active_window {
      ActiveWindowEnum::Typing => self.typing_window.handle_events(key),
      _ => false
    };

    event_occured
  }

  fn reset(&mut self) {
    self.time_secs = 0;
    self.typing_window.reset();
  }

  /// Exits running application
  ///
  /// Try to flush storage data
  fn exit(&mut self) {
    if let Some(storage_handler) = &self.storage_handler {
      storage_handler.flush().expect("Error occured while saving into the file");
    }

    self.is_exit = true;
  }

  fn switch_active_window(&mut self, switch_to_window: ActiveWindowEnum) {
    match switch_to_window {
      ActiveWindowEnum::Stats => self.typing_window.hide(),
      ActiveWindowEnum::Typing => self.stats_window.hide()
    }

    self.active_window = switch_to_window;
  }

  /// If the child window does not consume the event, check the keycodes
  fn handle_events(&mut self, key_event: KeyEvent) {
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
      match key_event.code {
        KeyCode::Char(c) => {
          match c {
            'r' => self.reset(),
            's' => {
              if let Some(storage_handler) = self.storage_handler.as_mut() {
                let layout_name_new = self.config.layout.switch_active_layout();
                storage_handler.switch_layout(layout_name_new);
                // println!("{:?}", storage_handler.get_data());
              }
            },
            'l' => self.switch_active_window(ActiveWindowEnum::Stats),
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
      self.exit();
    } else if key_event.code == KeyCode::Left {
      self.active_window = ActiveWindowEnum::Typing;
    } else if key_event.code == KeyCode::Right {
      self.active_window = ActiveWindowEnum::Stats;
    }
  }

}

