use crate::config::{AppConfig, TypingDuration};
use crate::event_handler::{EventHandler, TukaiEvent};
use crate::storage::storage_handler::StorageHandler;

use crate::screens::{
  Screen,
  typing_screen::TypingScreen,
  stats_screen::StatsScreen
};

use std::{
  rc::Rc,
  cell::RefCell
};

use ratatui::{
  crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
  layout::{Constraint, Layout},
  DefaultTerminal,
  Frame
};

#[derive(PartialEq, Hash, Eq)]
enum ActiveScreenEnum {
  Typing,
  Stats
}

pub struct App {
  // App config
  pub config: Rc<RefCell<AppConfig>>,

  // Storage handler
  storage_handler: Option<StorageHandler>,

  // App was interrupted
  is_exit: bool,

  // Time counter from start
  time_secs: u32,

  // Active screen
  active_window: ActiveScreenEnum,

  // Typing screen (ctrl-h)
  typing_window: TypingScreen,

  // Stats screen (ctrl-l)
  stats_window: StatsScreen
}

impl App {

  /// Creates new Tukai App
  pub fn new(config: AppConfig) -> Self {
    let config = Rc::new(RefCell::new(config));

    let typing_window = TypingScreen::new(Rc::clone(&config));
    let stats_window = StatsScreen::new(Rc::clone(&config));

    Self {
      config,

      storage_handler: None,

      is_exit: false,

      time_secs: 0,

      active_window: ActiveScreenEnum::Typing,

      typing_window,

      stats_window
    }
  }

  /// Returns the App config
  // fn get_config(&self) -> &Rc<RefCell<AppConfig>> {
  //   self.config
  // }

  /// Inits the App
  ///
  /// Storage handler (not reuired)
  pub fn init(mut self) -> Self {
    let config = self.config.clone();
    let mut config_mut = config.borrow_mut();

    match StorageHandler::new(&config_mut.get_file_path()).init() {
      Ok(storage_handler) => {
        let active_layout_name = storage_handler.get_active_layout_name().clone();
        config_mut.get_layout_mut().active_layout_name(active_layout_name);

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
  ) -> Result<(), Box<dyn std::error::Error>> {
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
      ActiveScreenEnum::Typing => {
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
        self.typing_window.render(
          frame,
          main_layout[0]);

        self.typing_window.render_instructions(frame, main_layout[1]);

        if self.typing_window.is_popup_visible() {
          self.typing_window.render_popup(frame);
        }
      },
      ActiveScreenEnum::Stats => {
        self.stats_window.render(
          frame,
          main_layout[0]);

        self.stats_window.render_instructions(frame, main_layout[1]);
      }
    }
  }

  fn handle_window_events(&mut self, key: KeyEvent) -> bool {
    let event_occured = match self.active_window {
      ActiveScreenEnum::Typing => self.typing_window.handle_events(key),
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

  fn switch_active_window(&mut self, switch_to_window: ActiveScreenEnum) {
    match switch_to_window {
      ActiveScreenEnum::Stats => self.typing_window.hide(),
      ActiveScreenEnum::Typing => self.stats_window.hide()
    }

    self.active_window = switch_to_window;
  }

  /// Switch the typing duration
  ///
  /// ThirtySec
  /// Minute
  /// ThreeMinutes
  pub fn switch_typing_duration(&mut self) {
    let mut config = self.config.borrow_mut();

    if config.typing_duration == TypingDuration::Minute {
      config.typing_duration = TypingDuration::ThreeMinutes;
    } else if config.typing_duration == TypingDuration::ThreeMinutes {
      config.typing_duration = TypingDuration::ThirtySec
    } else if config.typing_duration == TypingDuration::ThirtySec {
      config.typing_duration = TypingDuration::Minute
    }
  }

  /// If the child window does not consume the event, check the keycodes
  fn handle_events(&mut self, key_event: KeyEvent) {
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
      match key_event.code {
        KeyCode::Char(c) => {
          match c {
            'r' => self.reset(),
            't' => self.config.borrow_mut().toggle_transparent_bg(),
            'd' => {
              self.switch_typing_duration();
            },
            's' => {
              if let Some(storage_handler) = self.storage_handler.as_mut() {
                let layout_name_new = self.config.borrow_mut()
                  .get_layout_mut()
                  .switch_active_layout();

                storage_handler.switch_layout(layout_name_new);
              }
            },
            'l' => self.switch_active_window(ActiveScreenEnum::Stats),
            'h' => self.switch_active_window(ActiveScreenEnum::Typing),
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
      self.active_window = ActiveScreenEnum::Typing;
    } else if key_event.code == KeyCode::Right {
      self.active_window = ActiveScreenEnum::Stats;
    }
  }

}

