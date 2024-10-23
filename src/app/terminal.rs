use std::io;
use std::time::Duration;
use crate::app::windows::ActiveWindowEnum;

use ratatui::{DefaultTerminal, Frame};
use ratatui::crossterm::{self, event::{ self, KeyCode, KeyModifiers }};

pub struct App {
  is_exit: bool,
  active_window: ActiveWindowEnum,
}

impl App {
  
  pub fn new() -> Self {
    Self {
      is_exit: false,
      active_window: ActiveWindowEnum::Typing
    }
  }

  pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while self.is_exit {
      terminal.draw(|frame| self.draw(frame))?;
      self.handle_events()?;
      std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
  }

  pub fn draw(&mut self, frame: &mut Frame) {

  }

  fn handle_events(&mut self) -> io::Result<()> {
    if crossterm::event::poll(Duration::from_millis(100))? {
      if let event::Event::Key(key) = event::read()? {
        if key.code == KeyCode::Char('1') {
          self.active_window = ActiveWindowEnum::Typing;
        } else if key.code == KeyCode::Char('2') {
          self.active_window = ActiveWindowEnum::Settings;
        } else if key.code == KeyCode::Char('3') {
          self.active_window = ActiveWindowEnum::Results;
        }

        // self.handle_window_events(key);
      }
    }

    Ok(())
  }

}
