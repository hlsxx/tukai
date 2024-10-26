use ratatui::text::{Line, Span};

pub struct Generator {
}

impl Generator {
  pub fn generate_random_string() -> String {
    String::from("Hello car is blue white")
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
