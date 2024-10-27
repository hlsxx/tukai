use std::fs::File;
use rand::seq::SliceRandom;
use std::io::{self, BufRead};

use ratatui::text::{Line, Span};

pub struct Generator {
}

impl Generator {
  pub fn generate_random_string(size: usize) -> String {
    let mut words = Vec::new();

    if let Ok(file) = File::open("/mnt/holes/rust/tukaj/words/en.txt") {
      for line in io::BufReader::new(file).lines() {
        if let Ok(word) = line {
          words.push(word);
        }
      }
    }

    let mut rng = rand::thread_rng();

    let text = words.choose_multiple(&mut rng, size)
      .fold(String::new(), |mut acc, c| {
        acc.push_str(format!("{} ", c).as_str());
        acc
      });

    text
  }

  // pub fn generate_random_text() -> Vec<Line<'static>> {
  //   // let generated_string = Generator::generate_string();
  //
  //   let text = vec![
  //     Line::from(vec![
  //       Span::raw("First"),
  //       ".".into(),
  //     ]),
  //     Line::from("Second line"),
  //     "Third line".into(),
  //   ];
  //
  //   text
  // }

}
