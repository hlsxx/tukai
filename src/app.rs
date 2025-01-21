use crate::config::TukaiConfig;
use crate::event_handler::{EventHandler, TukaiEvent};
use crate::storage::storage_handler::StorageHandler;

use crate::screens::{stats_screen::StatsScreen, typing_screen::TypingScreen, Screen};

use std::{cell::RefCell, rc::Rc};

use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use ratatui::{
  crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
  layout::{Constraint, Layout},
  Frame,
};

#[derive(PartialEq, Hash, Eq)]
enum ActiveScreenEnum {
  Typing,
  Stats,
}

type TukaiTerminal = Terminal<CrosstermBackend<std::io::Stdout>>;

pub struct Tukai<'a> {
  // App config
  pub config: Rc<RefCell<TukaiConfig>>,

  // Cathing crossterm keyboard events
  event_handler: &'a mut EventHandler,

  // Storage handler
  storage_handler: StorageHandler,

  // App was interrupted
  is_exit: bool,

  screen: Box<dyn Screen>
}

impl<'a> Tukai<'a> {
  /// Attempts to create a new Tukai application.
  /// Tries to initialize `StorageHandler` then load
  /// an existing saved settings file.
  pub fn try_new(
    event_handler: &'a mut EventHandler,
    mut config: TukaiConfig,
  ) -> Result<Self, Box<dyn std::error::Error>> {
    // Inits storage handler
    let storage_handler = StorageHandler::new(config.get_file_path()).init()?;

    config.typing_duration = storage_handler.get_typing_duration();
    config.has_transparent_bg = storage_handler.get_has_transparent_bg();

    {
      let mut layout = config.get_layout_mut();
      layout.active_layout_name(storage_handler.get_layout_name().clone());
    }

    {
      let mut language = config.get_language_mut();
      language.current_index(storage_handler.get_language_index());
    }

    let config = Rc::new(RefCell::new(config));

    let typing_screen = TypingScreen::new(Rc::clone(&config));
    // let stats_screen = StatsScreen::new(Rc::clone(&config));

    Ok(Self {
      config,

      event_handler,

      storage_handler,

      is_exit: false,

      screen: Box::new(typing_screen)
    })
  }

  /// Runs and renders tui components.
  ///
  /// Handles events from `EventHandler`
  /// Handles tick (seconds, it's time counter) from `EventHandler`
  pub async fn run(
    &mut self,
    terminal: &mut TukaiTerminal,
  ) -> Result<(), Box<dyn std::error::Error>> {
    while !self.is_exit {
      match self.event_handler.next().await? {
        TukaiEvent::Key(key_event) => self.handle_events(key_event),
        TukaiEvent::Tick => {
          if self.screen.is_running() {
            self.screen.increment_time_secs();
          }
        }
      };

      terminal.draw(|frame| self.draw(frame))?;
    }

    Ok(())
  }

  /// Render tui components for a current screen
  fn draw(&mut self, frame: &mut Frame) {
    let main_layout = Layout::default()
      .constraints(vec![Constraint::Min(0), Constraint::Length(3)])
      .split(frame.area());

    self.screen.render(frame, main_layout[0]);

    // if self.screen.is_popup_visible() {
    //   if self.typing_screen.is_active() {
    //     self.typing_screen.toggle_active();
    //   }
    // } else if !self.typing_screen.is_active() {
    //   self.typing_screen.toggle_active();
    // }

    if self.screen.get_remaining_time() == 0 {
      self.screen.stop(&mut self.storage_handler);
    }

    // Renders
    self.screen.render(frame, main_layout[0]);
    self.screen.render_instructions(frame, main_layout[1]);

    if self.screen.is_popup_visible() {
      self.screen.render_popup(frame);
    }
  }

  fn handle_screen_events(&mut self, key: KeyEvent) -> bool {
    self.screen.handle_events(key)
  }

  fn reset(&mut self) {
    self.screen.reset();
  }

  /// Exits the running application
  ///
  /// Attempts to flush storage data before sets the `is_exit`.
  fn exit(&mut self) {
    self.is_exit = true;
    self
      .storage_handler
      .flush()
      .expect("Error occured while saving into the file");
  }

  /// Switches active `screen`.
  ///
  /// Hides the currently active screen.
  /// Sets the `active_screen` to the switched screen
  fn switch_active_screen(&mut self, switch_to_screen: ActiveScreenEnum) {
    match switch_to_screen {
      ActiveScreenEnum::Stats => {
        self.screen = Box::new(StatsScreen::new(self.config.clone()));
      },
      ActiveScreenEnum::Typing => {
        self.screen = Box::new(TypingScreen::new(self.config.clone()));
      }
    }
  }

  /// Handles crossterm events.
  ///
  /// First, checks for events with the pressed control button.
  /// Then, handles `screen` events (TypingScreen).
  /// Finally, processes remainig keys.
  fn handle_events(&mut self, key_event: KeyEvent) {
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
      match key_event.code {
        KeyCode::Char(c) => match c {
          'r' => self.reset(),
          'l' => self.switch_active_screen(ActiveScreenEnum::Stats),
          'h' => self.switch_active_screen(ActiveScreenEnum::Typing),
          'c' => self.exit(),
          'd' => {
            self
              .storage_handler
              .set_typing_duration(self.config.borrow_mut().switch_typing_duration());

            self.reset();
          }
          't' => {
            let new_state = self.config.borrow_mut().toggle_transparent_bg();
            self.storage_handler.set_transparent_bg(new_state);
          }
          's' => {
            let new_layout = self
              .config
              .borrow_mut()
              .get_layout_mut()
              .switch_to_next_layout();

            self.storage_handler.set_layout(new_layout);
          }
          'p' => {
            // switches language
            let new_language_index = self
              .config
              .borrow_mut()
              .get_language_mut()
              .switch_language();

            // saved into the storage
            self.storage_handler.set_language_index(new_language_index);

            self.reset();
          }
          _ => {}
        },
        _ => {}
      }

      return;
    }

    if self.handle_screen_events(key_event) {
      return;
    }

    if key_event.code == KeyCode::Esc {
      self.exit();
    } else if key_event.code == KeyCode::Left {
      self.switch_active_screen(ActiveScreenEnum::Typing)
    } else if key_event.code == KeyCode::Right {
      self.switch_active_screen(ActiveScreenEnum::Stats)
    }
  }
}
