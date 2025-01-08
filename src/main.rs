mod app;
mod config;
mod file_handler;

mod screens;
mod event_handler;
mod layout;
mod storage;
mod helper;

use event_handler::EventHandler;
use config::AppConfigBuilder;
use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut terminal = ratatui::init();
  let mut event_handler = EventHandler::new();

  let app_config = AppConfigBuilder::new()
    .build();

  terminal.clear()?;

  let app_result = App::try_new(app_config)?
    .run(&mut event_handler, &mut terminal)
    .await;

  ratatui::restore();
  app_result
}
