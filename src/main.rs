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
mod storage;
mod file_handler;

use core::error;
use std::path::PathBuf;
use config::Config;
use event_handler::EventHandler;
use file_handler::FileHandler;
use terminal::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  // let config_package = Config::get_package()?;
  // let mut terminal = ratatui::init();
  let mut event_handler = EventHandler::new();
  let mut file_handler = FileHandler::write_bytes_into_file(PathBuf::from("test.tukai"), &[0xA, 0xB]);

  Ok(())
  //
  // terminal.clear()?;
  //
  // let app_result = App::new(config_package)
  //   .run(&mut event_handler, &mut terminal)
  //   .await;
  //
  // ratatui::restore();
  // app_result
}
