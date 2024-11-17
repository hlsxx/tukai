use ratatui::{layout::Alignment, widgets::block::{Position, Title}};

pub fn get_title(layout_name: &str, window_name: &str) -> Title<'static> {
  Title::from(format!("⌨ tukai v0.0.2 》{} 》{} ⌨", layout_name, window_name))
    .position(Position::Top)
    .alignment(Alignment::Left)
}
