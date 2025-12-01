use std::env;

pub mod utils;
pub struct Day {
  pub year: u32,
  pub day: u32,
  pub run: fn(),
}

inventory::collect!(Day);

mod aoc_2025;
mod aoc_2024;

fn main() {
  let args: Vec<String> = env::args().collect();
  let year: u32 = args[1].parse().expect("number");
  let day: u32 = args[2].parse().expect("number");

  let maybe: Option<&Day> = inventory::iter::<Day>
    .into_iter()
    .find(|p| p.year == year && p.day == day);

  match maybe {
    Some(p) => (p.run)(),
    None => println!("No implementation for year {} day {}", year, day),
  }
}
