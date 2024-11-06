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
enum LayoutType {
  Classic,
  Anime,
  Premium
}

pub struct LayoutColors {
  primary: RgbColor,
  secondary: RgbColor,
  text: RgbColor,
  text_reverse: RgbColor,
  background: RgbColor,
  error: RgbColor
}

pub struct Layout {
  layouts: HashMap<LayoutType, LayoutColors>,
  active_layout_type: LayoutType
}

impl Layout {
  pub fn default() -> Self {
    let classic = LayoutColors {
      primary: (108, 181, 230),
      secondary: (232, 232, 232),
      text: (232, 232, 232),
      text_reverse: (125, 184, 209),
      background: (37, 40, 46),
      error: (255, 38, 38),
    };

    let anime = LayoutColors {
      primary: (216, 175, 193),
      secondary: (207, 147, 150),
      text: (237, 237, 237),
      text_reverse: (207, 147, 150),
      background: (81, 104, 125),
      error: (44, 56, 65),
    };

    let premium = LayoutColors {
      primary: (224, 174, 9),
      secondary: (117, 91, 5),
      text: (237, 237, 237),
      text_reverse: (37, 41, 47),
      background: (41, 41, 36),
      error: (224, 9, 9),
    };

    let mut layouts = HashMap::new();

    layouts.insert(LayoutType::Classic, classic);
    layouts.insert(LayoutType::Anime, anime);
    layouts.insert(LayoutType::Premium, premium);

    Self {
      layouts,
      active_layout_type: LayoutType::Classic
    }
  }

  pub fn switch_active_layout(&mut self) {
    if self.active_layout_type == LayoutType::Classic {
      self.active_layout_type = LayoutType::Anime;
    } else if self.active_layout_type == LayoutType::Anime {
      self.active_layout_type = LayoutType::Premium;
    } else {
      self.active_layout_type = LayoutType::Classic;
    }
  }

  fn get_layout_colors(&self) -> &LayoutColors {
    self.layouts.get(&self.active_layout_type).unwrap()
  }

  pub fn get_primary_color(&self) -> Color {
    self.get_layout_colors().primary.to_color()
  }

  pub fn get_secondary_color(&self) -> Color {
    self.get_layout_colors().secondary.to_color()
  }

  pub fn get_text_color(&self) -> Color {
    self.get_layout_colors().text.to_color()
  }

  pub fn get_text_reverse_color(&self) -> Color {
    self.get_layout_colors().text_reverse.to_color()
  }

  pub fn get_error_color(&self) -> Color {
    self.get_layout_colors().error.to_color()
  }

  pub fn get_background_color(&self) -> Color {
    self.get_layout_colors().background.to_color()
  }
}
