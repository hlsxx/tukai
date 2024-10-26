use std::fs::File;
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

    println!("{:?}", words.len());

    let mut text = String::new();

    for i in 0..size {
      text.push_str(&format!("{} ", &words[i]).to_string());
    }

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
