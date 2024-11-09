use ratatui::{layout::Alignment, widgets::block::{Position, Title}};

pub fn get_title() -> Title<'static> {
  Title::from("⌨ tukai v0.0.1 ⌨")
    .position(Position::Top)
    .alignment(Alignment::Left)
}
