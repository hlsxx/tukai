use std::collections::HashMap;

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::traits::ToColor;

/// Type alias for representing an RGB color as a tuple
type RgbColor = (u8, u8, u8);

impl ToColor for RgbColor {
  fn to_color(self) -> Color {
    Color::Rgb(self.0, self.1, self.2)
  }
}

#[allow(dead_code)]
pub enum LayoutColorTypeEnum {
  Primary,
  Secondary,
  Text,
  TextReverse,
  Background,
  Error
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum LayoutName {
  Neptune,
  Rust,
  Anime,
  Deadpool,
  Wolverine
}

pub struct LayoutColors {
  primary: RgbColor,
  text: RgbColor,
  text_current: RgbColor,
  text_current_bg: RgbColor,
  background: RgbColor,
  error: RgbColor
}

pub struct Layout {
  layouts: HashMap<LayoutName, LayoutColors>,
  active_layout_name: LayoutName
}

impl Layout {
  pub fn default() -> Self {
    let neptune = LayoutColors {
      primary: (108, 181, 230),
      text: (232, 232, 232),

      text_current: (25, 74, 107),
      text_current_bg: (200, 200, 200),

      background: (37, 40, 46),
      error: (214, 90, 90),
    };

    let rust = LayoutColors {
      primary: (150, 63, 17),
      text: (255, 178, 137),

      text_current: (23,23,23),
      text_current_bg: (210, 210, 210),

      background: (24, 8, 2),
      error: (200, 200, 200),
    };

    let deadpool = LayoutColors {
      primary: (139,35,35),
      text: (210, 210, 210),

      text_current: (23,23,23),
      text_current_bg: (210, 210, 210),

      background: (33, 29, 29),
      error: (110, 110, 110),
    };

    let wolverine = LayoutColors {
      primary: (196, 166, 51),
      text: (200, 200, 200),

      text_current: (23,23,23),
      text_current_bg: (210, 210, 210),

      background: (10, 14, 18),
      error: (110, 110, 110),
    };

    let anime = LayoutColors {
      primary: (216, 175, 193),

      text: (237, 237, 237),
      text_current: (50, 50, 50),
      text_current_bg: (202, 175, 216),

      background: (81, 104, 125),
      error: (44, 56, 65),
    };

    let mut layouts = HashMap::new();

    layouts.insert(LayoutName::Neptune, neptune);
    layouts.insert(LayoutName::Rust, rust);
    layouts.insert(LayoutName::Deadpool, deadpool);
    layouts.insert(LayoutName::Wolverine, wolverine);
    layouts.insert(LayoutName::Anime, anime);

    Self {
      layouts,
      active_layout_name: LayoutName::Neptune
    }
  }

  pub fn active_layout_name(&mut self, active_layout_name: LayoutName) {
    self.active_layout_name = active_layout_name;
  }

  pub fn get_active_layout_title(&self) -> &str {
    match self.active_layout_name {
      LayoutName::Neptune => "Neptune",
      LayoutName::Rust => "🦀 Rust",
      LayoutName::Anime => "🌸 Anime",
      LayoutName::Deadpool => "🩸🔞 Deadpool",
      LayoutName::Wolverine => "💪🍺 Wolverine"
    }
  }

  pub fn switch_active_layout(&mut self) -> LayoutName {
    if self.active_layout_name == LayoutName::Neptune {
      self.active_layout_name = LayoutName::Rust;
    } else if self.active_layout_name == LayoutName::Rust {
      self.active_layout_name = LayoutName::Anime;
    } else if self.active_layout_name == LayoutName::Anime {
      self.active_layout_name = LayoutName::Deadpool;
    } else if self.active_layout_name == LayoutName::Deadpool {
      self.active_layout_name = LayoutName::Wolverine;
    } else {
      self.active_layout_name = LayoutName::Neptune;
    }

    self.active_layout_name.clone()
  }

  fn get_layout_colors(&self) -> &LayoutColors {
    self.layouts.get(&self.active_layout_name).unwrap()
  }

  pub fn get_primary_color(&self) -> Color {
    self.get_layout_colors().primary.to_color()
  }

  pub fn get_text_color(&self) -> Color {
    self.get_layout_colors().text.to_color()
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
