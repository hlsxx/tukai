use std::{collections::HashMap, fmt::Display, hash::Hash};

use maplit::hashmap;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

pub trait ToColor {
  /// Converts the `(u8, u8, u8)` tuple to a `Color::Rgb`
  ///
  /// # Example
  ///
  /// ```
  /// use ratatui::style::Color
  ///
  /// let rgb: (u8, u8, u8) = (128, 64, 255);
  /// let color = rgb.to_color();
  ///
  /// assert_eq!(color, Color::Rgb(128, 64, 255));
  /// ```
  fn to_color(self) -> Color;
}

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
  Default,
  Neptune,
  Rust,
  Anime,
  Deadpool,
  Wolverine
}

impl Display for LayoutName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    use LayoutName::*;

    let display_text = match self {
      Default => "Default",
      Neptune => "Neptune",
      Rust => "🦀 Rust",
      Anime => "🌸 Anime",
      Deadpool => "🩸🔞 Deadpool",
      Wolverine => "💪🍺 Wolverine"
    };

    write!(f, "{}", display_text)
  }
}

pub struct LayoutColors {
  primary: RgbColor,
  text: RgbColor,
  text_current: RgbColor,
  text_current_bg: RgbColor,
  background: Option<RgbColor>,
  error: RgbColor
}

impl LayoutColors {
  pub fn new(
    primary: RgbColor,
    text: RgbColor,
    text_current: RgbColor,
    text_current_bg: RgbColor,
    background: Option<RgbColor>,
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
      Default => {
        LayoutColors::new(
         (125, 181, 114),
         (200, 200, 200),
         (25, 74, 107),
         (200, 200, 200),
         None,
         (179, 80, 80),
        )
      },
      Neptune => {
        LayoutColors::new(
         (108, 181, 230),
         (232, 232, 232),
         (25, 74, 107),
         (200, 200, 200),
         Some((37, 40, 46)),
         (214, 90, 90),
        )
      },
      Anime => {
        LayoutColors::new(
          (152, 117, 201),
          (222, 135, 174),
          (49, 45, 51),
          (222, 170, 146),
          Some((31, 27, 30)),
          (227, 138, 138),
        )
      },
      Deadpool => {
        LayoutColors::new(
          (139, 35, 35),
          (210, 210, 210),
          (23, 23, 23),
          (210, 210, 210),
          Some((33, 29, 29)),
          (110, 110, 110),
        )
      },
      Wolverine => {
        LayoutColors::new(
          (196, 166, 51),
          (200, 200, 200),
          (23,23,23),
          (210, 210, 210),
          Some((10, 14, 18)),
          (110, 110, 110),
        )
      },
      Rust => {
        LayoutColors::new(
          (150, 63, 17),
          (255, 178, 137),
          (255, 178, 137),
          (150, 63, 17),
          Some((24, 8, 2)),
          (120, 120, 120),
        )
      }
    };

    let transitions = HashMap::from([
      (Default, Neptune),
      (Neptune, Anime),
      (Anime, Deadpool),
      (Deadpool, Wolverine),
      (Wolverine, Rust),
      (Rust, Default)
    ]);

    Self {
      layouts,
      transitions,
      active_layout_name: LayoutName::Default
    }
  }

  /// Returns the currect active layout name
  pub fn get_active_layout_name(&self) -> &LayoutName {
    &self.active_layout_name
  }

  /// Sets a new active layout name
  pub fn active_layout_name(&mut self, active_layout_name: LayoutName) {
    self.active_layout_name = active_layout_name;
  }

  pub fn switch_active_layout(&mut self) -> LayoutName {
    if let Some(next_layout_name) = self.transitions.get(&self.active_layout_name) {
      self.active_layout_name = next_layout_name.clone();
    };

    self.active_layout_name.clone()
  }

  /// Returns the layout colors
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

  pub fn get_background_color(&self) -> Option<Color> {
    if let Some(bg_color)= self.get_layout_colors().background {
      Some(bg_color.to_color())
    } else {
      None
    }
  }
}
