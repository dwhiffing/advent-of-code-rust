fn part_1(ranges: &Vec<Vec<u64>>, ids: Vec<u64>) -> u64 {
  let mut count = 0;
  for id in ids {
    for r in ranges {
      if r[0] <= id && r[1] >= id { count += 1; break }
    }
  }
  count
}

fn part_2(ranges: &Vec<Vec<u64>>) -> u64 {
  let mut count = 0;
  for range in ranges {
    count += range[1] - range[0] + 1;
  }
  count
}

fn get_normalized_ranges(mut ranges: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
  ranges.sort_by(|a,b| a[0].cmp(&b[0]));
  
  let mut norm: Vec<Vec<u64>> = vec![];
  for i in 0..ranges.len()  {
    let l = norm.len();
    if l > 0 { 
      if ranges[i][1] <= norm[l-1][1] { continue; }
      if ranges[i][0] <= norm[l-1][1] + 1 { 
        norm[l-1][1] = ranges[i][1];
        continue; 
      }
    }
    norm.push(ranges[i].clone());
  }

  norm
}

pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("aoc_2025/05");
  let blank_index = lines.iter().position(|f| f == "").unwrap();
  let mut ranges: Vec<Vec<u64>> = lines[0..blank_index]
    .iter()
    .map(|line| 
      line.split("-").map(|f| f.parse::<u64>().unwrap()).collect()
    )
    .collect();

  ranges = get_normalized_ranges(ranges);

  let ids: Vec<u64> = lines[blank_index + 1..lines.len()]
    .iter()
    .map(|f| f.parse::<u64>().unwrap())
    .collect();

  let part1: u64 = part_1(&ranges, ids);
  let part2: u64 = part_2(&ranges);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
} 

inventory::submit! { crate::Day { year: 2025, day: 5, run } }

// 661, 359526404143208