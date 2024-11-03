use std::fs::File;
use rand::seq::SliceRandom;
use std::io::{self, BufRead};

pub struct Generator {
}

impl Generator {
  pub fn generate_random_string(size: usize) -> String {
    let mut words = Vec::new();

    if let Ok(file) = File::open("words/en.txt") {
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

}
