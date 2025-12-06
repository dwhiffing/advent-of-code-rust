type Map = Vec<Vec<char>>;

fn get_roll_count(map: &Map) -> usize {
  return map.iter().flatten().filter(|c| **c == '@').count();
}

fn get_roll_at(map: &Map, x: i32, y: i32) -> bool {
  let mx = (map.len() - 1) as i32;
  let my = (map[0].len() - 1) as i32;
  if x < 0 || y < 0 || x > mx || y > my { return false }
  map[y as usize][x as usize] == '@'
}

fn get_adjacent_roll_count(map: &Map, _x: usize, _y: usize) -> i32 {
  let x = _x as i32;
  let y = _y as i32;
  let mut n = 0;

  if get_roll_at(&map, x - 1, y) { n += 1 }; // left
  if get_roll_at(&map, x + 1, y) { n += 1 }; // right
  if get_roll_at(&map, x, y - 1) { n += 1 }; // up
  if get_roll_at(&map, x, y + 1) { n += 1 }; // down
  if get_roll_at(&map, x - 1, y - 1) { n += 1 }; // up left
  if get_roll_at(&map, x + 1, y - 1) { n += 1 }; // up right
  if get_roll_at(&map, x - 1, y + 1) { n += 1 }; // down left
  if get_roll_at(&map, x + 1, y + 1) { n += 1 }; // down right
  n
}

fn get_accessible_roll_count(map: &Map) -> u64 {
  let mut count: u64 = 0;

  for (y, line) in map.iter().enumerate() {
    for (x, c) in line.iter().enumerate() {
      if *c == '@' && get_adjacent_roll_count(&map, x, y) < 4 {
        count += 1;
      }
    }
  }
  
  return count;
} 

fn get_total_accessible_roll_count(mut map: Map) -> u64 {
  let original_count = get_roll_count(&map);
  let mut last_count = 0;
  
  loop {
    for (y, line) in map.clone().into_iter().enumerate() {
      for (x, c) in line.into_iter().enumerate() {
        if c == '@' && get_adjacent_roll_count(&map, x, y) < 4 {
          map[y][x] = '.'
        }
      }
    }
    let count = get_roll_count(&map);
    if count == last_count { break }
    last_count = count;
  }

  return (original_count - last_count) as u64;
} 


pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("aoc_2025/04");
  let map: Map = lines.iter().map(|l| l.chars().collect()).collect();
  let part1: u64 = get_accessible_roll_count(&map);
  let part2: u64 = get_total_accessible_roll_count(map);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
} 

inventory::submit! { crate::Day { year: 2025, day: 4, run } }
