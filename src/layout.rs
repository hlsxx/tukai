use std::collections::HashMap;

use ratatui::style::Color;

use crate::traits::ToColor;

/// Type alias for representing an RGB color as a tuple
type RgbColor = (u8, u8, u8);

impl ToColor for RgbColor {
  fn to_color(self) -> Color {
    Color::Rgb(self.0, self.1, self.2)
  }
}

pub enum LayoutColorTypeEnum {
  Primary,
  Secondary,
  Text,
  TextReverse,
  Background,
  Error
}

#[derive(PartialEq, Eq, Hash)]
pub enum LayoutType {
  Venus,
  Neptune,
  Anime,
  Premium
}

pub struct LayoutColors {
  primary: RgbColor,
  text: RgbColor,
  text_done: RgbColor,
  text_current: RgbColor,
  text_current_bg: RgbColor,
  background: RgbColor,
  error: RgbColor
}

pub struct Layout {
  layouts: HashMap<LayoutType, LayoutColors>,
  active_layout_type: LayoutType
}

impl Layout {
  pub fn default() -> Self {
    let venus = LayoutColors {
      primary: (207, 112, 154),

      text: (207, 112, 154),
      text_done: (156, 86, 153),

      text_current: (100, 100, 100),
      text_current_bg: (50, 50, 50),

      background: (252, 252, 252),
      error: (214, 90, 90),
    };

    let neptune = LayoutColors {
      primary: (108, 181, 230),
      text: (232, 232, 232),

      text_done: (232, 232, 232),
      text_current: (25, 74, 107),
      text_current_bg: (200, 200, 200),

      background: (37, 40, 46),
      error: (214, 90, 90),
    };

    let anime = LayoutColors {
      primary: (216, 175, 193),
      text_done: (207, 147, 150),

      text: (237, 237, 237),
      text_current: (50, 50, 50),
      text_current_bg: (202, 175, 216),

      background: (81, 104, 125),
      error: (44, 56, 65),
    };

    let mut layouts = HashMap::new();

    layouts.insert(LayoutType::Venus, venus);
    layouts.insert(LayoutType::Neptune, neptune);
    layouts.insert(LayoutType::Anime, anime);

    Self {
      layouts,
      active_layout_type: LayoutType::Neptune
    }
  }

  pub fn switch_active_layout(&mut self) {
    if self.active_layout_type == LayoutType::Neptune {
      self.active_layout_type = LayoutType::Venus;
    } else if self.active_layout_type == LayoutType::Venus {
      self.active_layout_type = LayoutType::Anime;
    } else {
      self.active_layout_type = LayoutType::Neptune;
    }
  }

  fn get_layout_colors(&self) -> &LayoutColors {
    self.layouts.get(&self.active_layout_type).unwrap()
  }

  pub fn get_primary_color(&self) -> Color {
    self.get_layout_colors().primary.to_color()
  }

  pub fn get_text_color(&self) -> Color {
    self.get_layout_colors().text.to_color()
  }

  pub fn get_text_done_color(&self) -> Color {
    self.get_layout_colors().text_done.to_color()
  }

  pub fn get_text_current_color(&self) -> Color {
    self.get_layout_colors().text_current.to_color()
  }

  pub fn get_text_current_bg_color(&self) -> Color {
    self.get_layout_colors().text_current_bg.to_color()
  }

  pub fn get_error_color(&self) -> Color {
    self.get_layout_colors().error.to_color()
  }

  pub fn get_background_color(&self) -> Color {
    self.get_layout_colors().background.to_color()
  }
}
