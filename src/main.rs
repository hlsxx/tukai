mod terminal;
mod traits;
mod tools;
mod windows;
mod configs;
mod helper;
mod event_handler;
mod config;
mod layout;
mod widgets;
mod file_handler;
mod helpers;
mod storage;
mod common;

use core::error;
use config::Config;
use event_handler::EventHandler;
use terminal::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  let config_package = Config::get_package()?;
  let mut terminal = ratatui::init();
  let mut event_handler = EventHandler::new();

  terminal.clear()?;

  let app_result = App::new(config_package)
    .init()
    .run(&mut event_handler, &mut terminal)
    .await;

  ratatui::restore();
  app_result
}
