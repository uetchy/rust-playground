extern crate rand;

use rand::Rng;
use std::io;

#[allow(dead_code)]
fn in_int() -> i64 {
  let mut line: String = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  line.trim().parse().unwrap()
}

#[allow(dead_code)]
fn in_str() -> Vec<char> {
  let mut line: String = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  line.trim().chars().collect()
}

#[allow(dead_code)]
fn in_multi() -> Vec<i64> {
  let mut line: String = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  line
    .split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect()
}

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);
  println!("The secret number is: {}", secret_number);
  println!("Please input your guess.");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .ok()
    .expect("Failed to read line");

  println!("You guessed: {}", guess);
}
