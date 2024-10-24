mod terminal;
mod traits;
mod tools;
mod windows;
mod configs;

use std::io;
use terminal::App;

fn main() -> io::Result<()> {
  let mut terminal = ratatui::init();
  terminal.clear()?;
  let app_result = App::new().run(&mut terminal);
  ratatui::restore();
  app_result
}
