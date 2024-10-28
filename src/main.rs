mod terminal;
mod traits;
mod tools;
mod windows;
mod configs;
mod constants;
mod helper;

use std::io;
use terminal::App;

#[tokio::main]
async fn main() -> io::Result<()> {
  let mut terminal = ratatui::init();
  terminal.clear()?;

  // App
  let app_result = App::new().run(&mut terminal).await;

  ratatui::restore();
  app_result
}
