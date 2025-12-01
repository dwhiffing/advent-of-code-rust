const SIZE: i32 = 100;

fn get_zero_count(lines: &Vec<String>, include_spins: bool) -> i32 {
  let mut dial: i32 = 50;
  let mut count: i32 = 0;

  for line in lines {
    let dir: i32 = if line.starts_with("L") { -1 } else { 1 };
    let num: i32 = line[1..line.len()].parse().expect("number");
    let spins: i32 = num / SIZE;
    let remainder: i32 = num % SIZE;
    
    if include_spins { count += spins; }
    dial += remainder * dir;

    if dial == 0 || dial == SIZE {
      dial = 0;
      count += 1;
    } else if dial < 0 || dial > SIZE {
      if dial.abs() != remainder && include_spins { count += 1; }
      dial -= SIZE * dir;
    }
  }

  return count
} 

pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("01");
  let part1: i32 = get_zero_count(&lines, false);
  let part2: i32 = get_zero_count(&lines, true);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
} 

inventory::submit! { crate::Day { year: 2025, day: 1, run } }
