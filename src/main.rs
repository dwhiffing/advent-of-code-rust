use std::env;

pub mod utils;
mod days;

fn invalid() {
  println!("Invalid puzzle number");
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let num: i32 = args[1].parse().expect("number");

  let run: fn() = match num {
    1 => days::day01::run,
    2 => days::day02::run,
    3 => days::day03::run,
    _ => invalid,
  };

  run();
}
