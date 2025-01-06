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
  active_screen: ActiveScreenEnum,

  // Typing screen (ctrl-h)
  typing_screen: TypingScreen,

  // Stats screen (ctrl-l)
  stats_screen: StatsScreen
}

impl App {

  /// Creates new Tukai App
  pub fn new(config: AppConfig) -> Self {
    let config = Rc::new(RefCell::new(config));

    let typing_screen = TypingScreen::new(Rc::clone(&config));
    let stats_screen = StatsScreen::new(Rc::clone(&config));

    Self {
      config,

      storage_handler: None,

      is_exit: false,

      time_secs: 0,

      active_screen: ActiveScreenEnum::Typing,

      typing_screen,

      stats_screen
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
          if self.typing_screen.is_running() {
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

    match self.active_screen {
      ActiveScreenEnum::Typing => {
        if self.typing_screen.is_popup_visible() {
          if self.typing_screen.is_active() {
            self.typing_screen.toggle_active();
          }
        } else if !self.typing_screen.is_active() {
          self.typing_screen.toggle_active();
        }

        self.typing_screen.time_secs = self.time_secs;

        if self.typing_screen.get_remaining_time() == 0 {
          self.typing_screen.stop(self.storage_handler.as_mut());
        }

        // Renders
        self.typing_screen.render(
          frame,
          main_layout[0]);

        self.typing_screen.render_instructions(frame, main_layout[1]);

        if self.typing_screen.is_popup_visible() {
          self.typing_screen.render_popup(frame);
        }
      },
      ActiveScreenEnum::Stats => {
        self.stats_screen.render(
          frame,
          main_layout[0]);

        self.stats_screen.render_instructions(frame, main_layout[1]);
      }
    }
  }

  fn handle_screen_events(&mut self, key: KeyEvent) -> bool {
    let event_occured = match self.active_screen {
      ActiveScreenEnum::Typing => self.typing_screen.handle_events(key),
      _ => false
    };

    event_occured
  }

  fn reset(&mut self) {
    self.time_secs = 0;
    self.typing_screen.reset();
  }

  /// Exits the running application
  ///
  /// Attempts to flush storage data before sets the `is_exit`.
  fn exit(&mut self) {
    if let Some(storage_handler) = &self.storage_handler {
      storage_handler.flush().expect("Error occured while saving into the file");
    }

    self.is_exit = true;
  }

  /// Switches active `screen`.
  ///
  /// Hides the currently active screen.
  /// Sets the `active_screen` to the switched screen
  fn switch_active_screen(&mut self, switch_to_screen: ActiveScreenEnum) {
    match switch_to_screen {
      ActiveScreenEnum::Stats => self.typing_screen.hide(),
      ActiveScreenEnum::Typing => self.stats_screen.hide()
    }

    self.active_screen = switch_to_screen;
  }

  /// Switches the typing duration.
  ///
  /// Options:
  /// 1. Minute
  /// 2. Three minutes
  /// 3. Thirty seconds
  pub fn switch_typing_duration(&mut self) {
    let mut config = self.config.borrow_mut();

    config.typing_duration = match config.typing_duration {
      TypingDuration::Minute => TypingDuration::ThreeMinutes,
      TypingDuration::ThreeMinutes => TypingDuration::ThirtySec,
      TypingDuration::ThirtySec => TypingDuration::Minute,
    }
  }

  /// Handles crossterm events.
  ///
  /// First, checks for events with the pressed control button.
  /// Then, handles `screen` events (TypingScreen).
  /// Finally, processes remainig keys.
  fn handle_events(&mut self, key_event: KeyEvent) {
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
      let storage_handler = self.storage_handler.as_mut().unwrap();

      match key_event.code {
        KeyCode::Char(c) => {
          match c {
            'r' => self.reset(),
            'l' => self.switch_active_screen(ActiveScreenEnum::Stats),
            'h' => self.switch_active_screen(ActiveScreenEnum::Typing),
            'c' => self.exit(),
            'd' => {
              self.switch_typing_duration();
              self.reset();
            },
            't' => {
              let new_state = self.config.borrow_mut().toggle_transparent_bg();
              storage_handler.set_transparent_bg(new_state);
            },
            's' => {
              let new_layout = self.config.borrow_mut()
                .get_layout_mut()
                .switch_active_layout();

              storage_handler.set_layout(new_layout);
            }
            _ => {}
          }
        },
        _ => {}
      }

      return;
    }

    if self.handle_screen_events(key_event) { return; }

    if key_event.code == KeyCode::Esc {
      self.exit();
    } else if key_event.code == KeyCode::Left {
      self.active_screen = ActiveScreenEnum::Typing;
    } else if key_event.code == KeyCode::Right {
      self.active_screen = ActiveScreenEnum::Stats;
    }
  }

}

