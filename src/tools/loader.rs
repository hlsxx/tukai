pub struct Loader<'a> {
  items: Vec<&'a str>,
  current_item_index: usize
}

impl<'a> Loader<'a> {
  pub fn new() -> Self {
    let items = vec!["-", "\\", "|", "/"];

    Self {
      items,
      current_item_index: 0
    }
  }

  fn next(&mut self) {
    self.current_item_index = (self.current_item_index + 1) % self.items.len();
  }

  pub fn get_slash(&mut self) -> &'a str {
    self.next();
    self.items[self.current_item_index]
  }
}

