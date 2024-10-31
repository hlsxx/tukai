mod terminal;
mod traits;
mod tools;
mod windows;
mod configs;
mod constants;
mod helper;
mod event_handler;

use std::io;
use terminal::App;

fn main() -> io::Result<()> {
  let mut terminal = ratatui::init();
  terminal.clear()?;

  // App
  let app_result = App::new().run(&mut terminal);

  ratatui::restore();
  app_result
}
