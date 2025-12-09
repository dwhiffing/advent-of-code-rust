use itertools::Itertools;

type Coord = (i64, i64, i64);

fn dist(a: &Coord, b: &Coord) -> i64 {
  let dx = a.0 - b.0;
  let dy = a.1 - b.1;
  let dz = a.2 - b.2;
  dx * dx + dy * dy + dz * dz
}

fn find(circuits: &Vec<usize>, index: usize) -> usize {
  if circuits[index] == index { return index }
  find(circuits, circuits[index])
}

fn union(circuits: &mut Vec<usize>, a: usize, b: usize) {
  let a_root = find(circuits, a);
  let b_root = find(circuits, b);
  circuits[b_root] = a_root;
}

fn solve(coords: &[Coord], max_connections: usize) -> (Vec<usize>, (usize, usize)) {
  let n: usize = coords.len();
  let mut circuits: Vec<usize> = (0..n).collect();
  let mut last = (0, 0);
  
  let mut distances = Vec::new();
  for i in 0..n {
    for j in i+1..n {
      distances.push((i, j, dist(&coords[i], &coords[j])));
    }
  }
  distances.sort_by(|a, b| a.2.cmp(&b.2));
  
  for index in 0..max_connections {
    let (a, b, _dist) = distances[index];
    let circuit_count = circuits.iter().enumerate().filter(|(i, v)| *i == **v).count();
    if circuit_count == 1 { break }
    
    union(&mut circuits, a, b);
    last = (a, b);
  }

  (circuits, last)
}

pub fn part_1(coords: &Vec<Coord>) -> u64 {
  let (circuits, _) = solve(&coords, 1000);

  let mut sizes = vec![0; coords.len()];
  for i in 0..sizes.len() {
    sizes[find(&circuits, i)] += 1;
  }

  sizes.into_iter().sorted().rev().take(3).product()
}

pub fn part_2(coords: &Vec<Coord>) -> u64 {
  let (_, (i, j)) = solve(&coords, usize::MAX);
  (coords[i].0 as u64) * (coords[j].0 as u64)
}

pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("aoc_2025/08");
  let coords: Vec<Coord> = lines
    .iter()
    .map(|line| 
      line
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap()
    )
    .collect();
  
  let part1: u64 = part_1(&coords);
  let part2: u64 = part_2(&coords);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
}

inventory::submit! { crate::Day { year: 2025, day: 8, run } }
