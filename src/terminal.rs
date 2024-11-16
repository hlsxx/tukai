use crossterm::event::KeyModifiers;
use crate::configs::app_config::AppConfig;
use crate::event_handler::{EventHandler, TukaiEvent};
use crate::storage::storage_handler::StorageHandler;
use crate::windows::{
  typing_window::TypingWindow,
  stats_window::StatsWindow
};

use crate::layout::Layout as TukaiLayout;

use crate::traits::Window;

use std::error;
use ratatui::{
  crossterm::event::{KeyCode, KeyEvent},
  layout::{Constraint, Layout},
  DefaultTerminal,
  Frame
};

#[derive(PartialEq, Hash, Eq)]
enum ActiveWindowEnum {
  Typing,
  Stats
}


pub struct App {
  pub config: AppConfig,

  // version: Option<String>,

  storage_handler: Option<StorageHandler>,

  is_exit: bool,

  time_secs: u32,

  active_window: ActiveWindowEnum,

  // Windows
  typing_window: TypingWindow,

  stats_window: StatsWindow
}

impl App {

  /// Creates new Tukai App
  pub fn new(config: AppConfig) -> Self {
    Self {
      config,

      // version: None,

      storage_handler: None,

      is_exit: false,

      time_secs: 0,

      active_window: ActiveWindowEnum::Typing,

      typing_window: TypingWindow::default(),
      stats_window: StatsWindow::default()
    }
  }

  fn get_config_layout(&self) -> std::cell::Ref<TukaiLayout> {
    self.config.get_layout()
  }

  // fn get_version(&self) -> String {
  //   self.version.clone().unwrap_or("x.x.x".to_string())
  // }

  /// Inits the App
  ///
  /// Storage handler (not reuired)
  pub fn init(mut self) -> Self {
    match StorageHandler::new(&self.config.get_file_path()).init() {
      Ok(storage_handler) => {
        if let Some(active_layout_name) = storage_handler.get_active_layout_name() {
          self.config.get_layout_mut().active_layout_name(active_layout_name);
        }

        self.storage_handler = Some(storage_handler);
      },
      Err(_) => {}
    }

    // match FileHandler::read_bytes_from_file("Cargo.toml") {
    //   Ok(toml_data) => {
    //     let toml_config = str::from_utf8(&toml_data).unwrap();
    //     let tukai_config = toml::from_str::<Config>(&toml_config).unwrap();
    //     self.version = Some(tukai_config.package.version);
    //   },
    //   Err(_) => {}
    // }

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
                let layout_name_new = self.config.get_layout_mut().switch_active_layout();
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

