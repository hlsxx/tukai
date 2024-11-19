use ratatui::{layout::Alignment, widgets::block::{Position, Title}};

pub fn get_title(version: &String, layout_name: &str, window_name: &str) -> Title<'static> {
  Title::from(format!("⌨ tukai v{} 》{} 》{} ⌨", version, layout_name, window_name))
    .position(Position::Top)
    .alignment(Alignment::Left)
}
