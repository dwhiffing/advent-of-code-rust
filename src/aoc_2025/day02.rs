fn get_invalid_ids(lines: &Vec<String>, allow_gt_two: bool) -> u64 {
  let mut invalid_ids = vec![];
  
  let ranges: Vec<&str> = lines[0].split(",").collect();
  for range in ranges {
    let values: Vec<&str> = range.split("-").collect();
    let start: u64 = values[0].parse().expect("number");
    let end: u64 = values[1].parse().expect("number");

    for id in start ..= end {
      let as_string = id.to_string();
      let len = as_string.chars().count();
      
      if len % 2 != 0 && !allow_gt_two { continue }
      
      let min_size = if allow_gt_two { 1 } else { len / 2 };
      let max_size = len / 2;

      for size in min_size ..= max_size {
        let chars: Vec<char> = as_string.chars().collect();
        let all_chunks_equal = chars.chunks(size).all(|chunk| chunk == &chars[0..size]);
        if all_chunks_equal {
          invalid_ids.push(id);
          break;
        }
      }
    }
  }

  return invalid_ids.into_iter().sum()
} 

pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("aoc_2025/02");
  let part1: u64 = get_invalid_ids(&lines, false);
  let part2: u64 = get_invalid_ids(&lines, true);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
} 

inventory::submit! { crate::Day { year: 2025, day: 2, run } }
