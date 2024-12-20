mod app;
mod config;
mod file_handler;

mod screens;
mod event_handler;
mod layout;
mod storage;
mod helper;

use core::error;
use event_handler::EventHandler;
use layout::Layout as TukaiLayout;
use config::AppConfigBuilder;
use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  let mut terminal = ratatui::init();
  let mut event_handler = EventHandler::new();

  let app_config = AppConfigBuilder::new()
    .file_path("tukai.bin")
    .layout(TukaiLayout::default())
    .build();

  terminal.clear()?;

  let app_result = App::new(app_config)
    .init()
    .run(&mut event_handler, &mut terminal)
    .await;

  ratatui::restore();
  app_result
}
