use rand::{seq::SliceRandom, Rng};

pub struct Generator {}

impl Generator {

  /// Generates a random string of words
  ///
  /// This method generates a string containing random
  /// words from the words/en.txt file
  pub fn generate_random_string(amount: usize) -> String {
    let words_string = include_str!("../../words/en.txt");

    let words = words_string.lines()
      .map(|line| line)
      .collect::<Vec<&str>>();

    let mut rng = rand::thread_rng();

    let text = words.choose_multiple(&mut rng, amount * 2)
      .fold(String::new(), |mut acc, c| {
        acc.push_str(format!("{} ", c).as_str());
        acc
      });

    text
  }

  /// Generates a random motto for the block bottom title
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
