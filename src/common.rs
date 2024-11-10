use ratatui::{layout::Alignment, widgets::block::{Position, Title}};

pub fn get_title(window_name: &str) -> Title<'static> {
  Title::from(format!("⌨ tukai v0.0.1 》{} ⌨", window_name))
    .position(Position::Top)
    .alignment(Alignment::Left)
}
