use ratatui::style::Color;

pub fn get_color_rgb(color: (u8, u8, u8)) -> Color {
  Color::Rgb(color.0, color.1, color.2)
}
