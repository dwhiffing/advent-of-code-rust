use itertools::Itertools;

fn get_joltage(lines: &Vec<String>, size: usize) -> u64 {
  let mut joltages: Vec<u64> = vec![];
  
  for line in lines {
    let mut candidates = line
      .chars()
      .map(|c| c.to_digit(10).unwrap() as u64)
      .enumerate()
      .sorted_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap())
      .collect_vec();

    let mut joltage: u64 = 0;
    let mut last_index = 0;
    for k in 0..size {
      for (i, v) in candidates.clone() {
        if last_index != 0 && i <= last_index { continue }
        if line.len() - i < size - k { continue }

        joltage = joltage * 10 + v;
        last_index = i;
        candidates = candidates.into_iter().filter(|(ci, cv)| *ci != i || *cv != v).collect();
        break;
      }
    }

    joltages.push(joltage);
  }

  return joltages.iter().sum();
} 

pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("aoc_2025/03");
  let part1: u64 = get_joltage(&lines, 2);
  let part2: u64 = get_joltage(&lines, 12);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
} 

inventory::submit! { crate::Day { year: 2025, day: 3, run } }
