fn zip<T: Clone>(vecs: &Vec<Vec<T>>) -> Vec<Vec<T>> {
  let mut result: Vec<Vec<T>> = vec![vec![]; vecs[0].len()];

  for vec in vecs {
    for (i, v) in vec.iter().enumerate() {
      result[i].push(v.clone())
    }
  }

  result
}

fn answer_problems(problems: Vec<Vec<String>>) -> u64 {
  problems
    .iter()
    .map(|problem| {
      let op = problem.last().unwrap().chars().last().unwrap_or('+');

      let nums = problem
        .iter()
        .filter_map(|string| {
          let digits: String = string
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect();
          digits.parse::<u64>().ok()
        });

      if op == '+' { nums.sum::<u64>() } else { nums.product::<u64>() }
    })
    .sum()
}

fn part_1(lines: &Vec<String>) -> u64 {
  let matches = lines
    .iter()
    .map(|line| line.split_whitespace().map(str::to_string).collect())
    .collect();

  let problems = zip(&matches);

  answer_problems(problems)
}

fn part_2(lines: &Vec<String>) -> u64 {
  let chars: Vec<Vec<char>> = lines
    .iter()
    .map(|line| line.chars().collect())
    .collect();

  let columns: Vec<String> = zip(&chars)
    .iter()
    .rev()
    .map(|chars| chars.iter().collect())
    .collect();

  let problems = columns
    .split(|column| column.trim().is_empty())
    .map(|split| split.to_vec())
    .collect();

  answer_problems(problems)
}

pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("aoc_2025/06");
  let part1: u64 = part_1(&lines);
  let part2: u64 = part_2(&lines);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
} 

inventory::submit! { crate::Day { year: 2025, day: 6, run } }
