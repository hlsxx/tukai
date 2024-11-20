mod app;
mod windows;
mod configs;
mod event_handler;
mod layout;
mod file_handler;
mod storage;
mod helper;

use core::error;
use event_handler::EventHandler;
use layout::Layout as TukaiLayout;
use app::App;
use configs::app_config::AppConfigBuilder;

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
