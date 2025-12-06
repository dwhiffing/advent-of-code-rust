use regex::Regex;

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
  let non_digit_regex = Regex::new(r"\D").unwrap();
  
  problems
    .iter()
    .map(|problem| {
      let op = problem.last().unwrap().chars().last().unwrap_or('+');
      
      let nums = problem
        .iter()
        .filter_map(|s| 
          non_digit_regex.replace_all(s, "").parse::<u64>().ok()
        );
      
      if op == '+' { nums.sum::<u64>() } else { nums.product::<u64>() }
    })
    .sum()
}

fn part_1(lines: &Vec<String>) -> u64 {
  let non_whitespace_regex = Regex::new(r"\S+").unwrap();

  let matches = lines
    .iter()
    .map(|line| 
      non_whitespace_regex
        .find_iter(line)
        .map(|c| c.as_str().to_string())
        .collect()
    )
    .collect();

  answer_problems(zip(&matches))
}

fn part_2(lines: &Vec<String>) -> u64 {
  let chars: Vec<Vec<char>> = lines
    .iter()
    .map(|line| line.chars().collect() )
    .collect();

  let columns: Vec<String> = zip(&chars)
    .iter()
    .rev()
    .map(|r| r.iter().collect())
    .collect();

  let problems = columns
    .split(|f| f.trim().is_empty())
    .map(|s| s.to_vec())
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
