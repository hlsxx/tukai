mod terminal;
mod traits;
mod tools;
mod windows;
mod configs;
mod constants;
mod helper;
mod event_handler;

use core::error;
use event_handler::EventHandler;
use terminal::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  let mut terminal = ratatui::init();
  let mut event_handler = EventHandler::new();

  terminal.clear()?;

  let app_result = App::new()
    .run(&mut event_handler, &mut terminal)
    .await;

  ratatui::restore();
  app_result
}
