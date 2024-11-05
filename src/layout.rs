use std::collections::HashMap;

type RgbColor = (u8, u8, u8);

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
  text_secondary: RgbColor,
  background: RgbColor,
  error: RgbColor,
  error_secondary: RgbColor,
}

pub struct Layout {
  layouts: HashMap<LayoutType, LayoutColors>,
  current_layout: LayoutType
}

impl Layout {
  pub fn new() -> Self {
    let classic = LayoutColors {
      primary: (224, 174, 9),
      secondary: (117, 91, 5),
      text: (237, 237, 237),
      text_secondary: (37, 41, 47),
      background: (41, 41, 36),
      error: (224, 9, 9),
      error_secondary: (224, 9, 9)
    };

    let anime = LayoutColors {
      primary: (223, 218, 215),
      secondary: (223, 218, 215),
      text: (237, 237, 237),
      text_secondary: (37, 41, 47),
      background: (81, 104, 125),
      error: (37, 41, 47),
      error_secondary: (224, 9, 9)
    };

    let premium = LayoutColors {
      primary: (223, 218, 215),
      secondary: (223,218,215),
      text: (237, 237, 237),
      text_secondary: (37, 41, 47),
      background:(81, 104, 125),
      error: (37, 41, 47),
      error_secondary: (224, 9, 9)
    };

    let mut layouts = HashMap::new();

    layouts.insert(LayoutType::Classic, classic);
    layouts.insert(LayoutType::Anime, anime);
    layouts.insert(LayoutType::Premium, premium);

    Self {
      layouts,
      current_layout: LayoutType::Classic
    }
  }
}
