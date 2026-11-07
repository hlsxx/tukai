use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("alice", 42);
    scores.insert("bob", 17);
    scores.insert("carol", 33);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    let total: i32 = scores.values().sum();
    println!("total: {}", total);

    let doubled: Vec<i32> = scores.values().map(|s| s * 2).collect();
    println!("doubled: {:?}", doubled);
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.trim().parse::<i32>()
}

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().map(String::from).collect())
}

fn first_word(text: &str) -> Option<&str> {
    text.split_whitespace().next()
}

trait Drawable {
    fn draw(&self) -> String;
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("circle with radius {:.2}", self.radius)
    }
}

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }

    counts
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let squares: Vec<i32> = numbers.iter().map(|n| n * n).collect();
    let evens: Vec<i32> = numbers.iter().copied().filter(|n| is_even(*n)).collect();

    println!("squares: {:?}", squares);
    println!("evens: {:?}", evens);

    let point_a = Point::origin();
    let point_b = Point::new(3.0, 4.0);
    println!("distance: {}", point_a.distance(&point_b));

    let mut direction = Direction::North;
    direction = direction.turn_right();

    let number = parse_number("42")?;
    let biggest = max(number, 100);

    let text = "the quick brown fox jumps over the lazy dog";
    let word_counts = count_words(text);
    let first = first_word(text).unwrap_or("");

    let circle = Circle { radius: 5.0 };
    println!("{}", circle.draw());

    for i in 0..10 {
        println!("fib({}) = {}", i, fibonacci(i));
    }

    println!("biggest: {}, first: {}, words: {}", biggest, first, word_counts.len());

    Ok(())
}
