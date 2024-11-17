use std::fs::File;
use rand::{seq::SliceRandom, Rng};
use std::io::{self, BufRead};

pub struct Generator {
}

impl Generator {
  pub fn generate_random_string(amount: usize) -> String {
    let mut words = Vec::new();

    if let Ok(file) = File::open("words/en.txt") {
      for line in io::BufReader::new(file).lines() {
        if let Ok(word) = line {
          words.push(word);
        }
      }
    }

    let mut rng = rand::thread_rng();

    let text = words.choose_multiple(&mut rng, amount * 2)
      .fold(String::new(), |mut acc, c| {
        acc.push_str(format!("{} ", c).as_str());
        acc
      });

    text
  }

  pub fn generate_random_motto() -> String {
    let mottos = vec![
      "Practice today, master tomorrow",
      "Fingers on keys, progress with ease",
      "Consistency breeds accuracy",
      "Type smarter, not harder",
      "Precision today, perfection tomorrow"
    ];

    let mut rng = rand::thread_rng();

    let random_index = rng.gen_range(0..mottos.len());
  
    String::from(mottos[random_index])
  }

}
