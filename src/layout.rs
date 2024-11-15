use std::{collections::HashMap, io};

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::{storage::storage_handler::{self, StorageHandler}, traits::ToColor};

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

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
pub enum LayoutName {
  Neptune,
  Deadpool,
  Wolverine,
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
  layouts: HashMap<LayoutName, LayoutColors>,
  active_layout_name: LayoutName
}

impl Layout {
  pub fn default() -> Self {
    // let venus = LayoutColors {
    //   primary: (207, 112, 154),
    //
    //   text: (207, 112, 154),
    //   text_done: (156, 86, 153),
    //
    //   text_current: (100, 100, 100),
    //   text_current_bg: (50, 50, 50),
    //
    //   background: (252, 252, 252),
    //   error: (214, 90, 90),
    // };
    //
    let neptune = LayoutColors {
      primary: (108, 181, 230),
      text: (232, 232, 232),

      text_done: (232, 232, 232),
      text_current: (25, 74, 107),
      text_current_bg: (200, 200, 200),

      background: (37, 40, 46),
      error: (214, 90, 90),
    };

    let deadpool = LayoutColors {
      primary: (139,35,35),
      text: (210, 210, 210),

      text_done: (232, 232, 232),
      text_current: (23,23,23),
      text_current_bg: (210, 210, 210),

      background: (33, 29, 29),
      error: (110, 110, 110),
    };

    let wolverine = LayoutColors {
      primary: (196, 166, 51),
      text: (196, 166, 51),

      text_done: (232, 232, 232),
      text_current: (23,23,23),
      text_current_bg: (210, 210, 210),

      background: (10, 14, 18),
      error: (110, 110, 110),
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

    // layouts.insert(LayoutName::Venus, venus);
    layouts.insert(LayoutName::Neptune, neptune);
    layouts.insert(LayoutName::Deadpool, deadpool);
    layouts.insert(LayoutName::Wolverine, wolverine);
    layouts.insert(LayoutName::Anime, anime);

    Self {
      layouts,
      active_layout_name: LayoutName::Neptune
    }
  }

  // pub fn init(&mut self) -> Result<(), io::Error> {
  //   let storage_handler = StorageHandler::new("test.tukai")
  //     .init()?;
  //
  //   if let Some(layout_name) = storage_handler.get_active_layout_name() {
  //     self.active_layout_name = layout_name;
  //   }
  //
  //   Ok(())
  // }
  //

  pub fn active_layout_name(&mut self, active_layout_name: LayoutName) {
    self.active_layout_name = active_layout_name;
  }

  pub fn get_active_layout_title(&self) -> &str {
    match self.active_layout_name {
      LayoutName::Neptune => "Neptune",
      LayoutName::Anime => "Anime",
      LayoutName::Premium => "Premium",
      LayoutName::Deadpool => "🩸🔞 Deadpool",
      LayoutName::Wolverine => "💪🍺 Wolverine"
    }
  }

  pub fn switch_active_layout(&mut self) -> LayoutName {
    if self.active_layout_name == LayoutName::Neptune {
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
