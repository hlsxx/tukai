use std::{collections::HashMap, hash::Hash};

use maplit::hashmap;
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

impl LayoutColors {
  pub fn new(
    primary: RgbColor,
    text: RgbColor,
    text_current: RgbColor,
    text_current_bg: RgbColor,
    background: RgbColor,
    error: RgbColor,
  ) -> Self {
    Self {
      primary,
      text,
      text_current,
      text_current_bg,
      background,
      error
    }
  }
}

pub struct Layout {
  layouts: HashMap<LayoutName, LayoutColors>,
  transitions: HashMap<LayoutName, LayoutName>,
  active_layout_name: LayoutName
}

impl Layout {
  pub fn default() -> Self {
    use LayoutName::*;

    let layouts = hashmap! {
      Neptune => {
        LayoutColors::new(
         (108, 181, 230),
         (232, 232, 232),
         (25, 74, 107),
         (200, 200, 200),
         (37, 40, 46),
         (214, 90, 90),
        )
      },
      Anime => {
        LayoutColors::new(
          (216, 175, 193),
          (237, 237, 237),
          (50, 50, 50),
          (202, 175, 216),
          (81, 104, 125),
          (44, 56, 65),
        )
      },
      Deadpool => {
        LayoutColors::new(
          (139, 35, 35),
          (210, 210, 210),
          (23, 23, 23),
          (210, 210, 210),
          (33, 29, 29),
          (110, 110, 110),
        )
      },
      Wolverine => {
        LayoutColors::new(
          (196, 166, 51),
          (200, 200, 200),
          (23,23,23),
          (210, 210, 210),
          (10, 14, 18),
          (110, 110, 110),
        )
      },
      Rust => {
        LayoutColors::new(
          (150, 63, 17),
          (255, 178, 137),
          (255, 178, 137),
          (150, 63, 17),
          (24, 8, 2),
          (120, 120, 120),
        )
      }
    };

    let transitions = HashMap::from([
      (Neptune, Anime),
      (Anime, Deadpool),
      (Deadpool, Wolverine),
      (Wolverine, Rust),
      (Rust, Neptune),
    ]);

    Self {
      layouts,
      transitions,
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
    if let Some(next_layout_name) = self.transitions.get(&self.active_layout_name) {
      self.active_layout_name = next_layout_name.clone();
    };

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
