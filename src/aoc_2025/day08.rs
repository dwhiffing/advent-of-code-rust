use std::collections::{HashMap, HashSet};
use itertools::Itertools;

type Coord = (i64, i64, i64);
type Circuit = HashSet<Coord>;
type Connects = HashMap<Coord, Vec<Coord>>;
type Pairs = HashMap<Coord, Vec<(Coord, f64)>>;

fn dist(a: &Coord, b: &Coord) -> f64 {
  let x = (a.0 - b.0).abs().pow(2);
  let y = (a.1 - b.1).abs().pow(2);
  let z = (a.2 - b.2).abs().pow(2);
  let f = (x + y + z) as f64;
  f.sqrt()
}

fn already_connected(circuits: &Vec<Circuit>, a:&Coord, b:&Coord) -> bool {
  circuits.iter().any(|c| c.contains(a) && c.contains(b))
}

fn connect(circuits: &Vec<Circuit>, a:&Coord, b:&Coord) -> Vec<Circuit> {
  let mut cloned: Vec<HashSet<(i64, i64, i64)>> = circuits.clone();
  if already_connected(circuits, a, b) { return cloned}
  let mut a_index = circuits.iter().position(|c| c.contains(&a)).unwrap();
  let b_index = circuits.iter().position(|c| c.contains(&b)).unwrap();
  let b_set = cloned.remove(b_index);
  if b_index < a_index { a_index -= 1; }

  cloned[a_index].extend(b_set);
  cloned
}

fn get_closest_pairs(coords: &Vec<Coord>) -> Pairs {
  let mut hash = HashMap::new();
  for coord in coords {
    let _n: Vec<(Coord, f64)> = coords
      .iter()
      .map(|b| (*b, dist(coord, b)))
      .filter(|c| c.1 > 0.0)
      .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap() )
      .collect();

    hash.insert(*coord, _n);
  }

  hash
}

fn part_1(coords: &Vec<Coord>, pairs: &Pairs, n:u64) -> u64 {
  let mut circuits: Vec<Circuit> = vec![];
  let mut connects: Connects = HashMap::new();
  for coord in coords.clone() {
    circuits.push(HashSet::from([coord]));
    connects.insert(coord, vec![]);
  }

  for _i in 0..n {
    let (a, b, _dist) = pairs
      .iter()
      .map(|(k, v)| {
        let closest = v
          .iter()
          .find(|c| !&connects[k].contains(&c.0))
          .unwrap();
        (*k, closest.0, closest.1)
      })
      .min_by(|x, y| x.2.partial_cmp(&y.2).unwrap())
      .unwrap();

    // println!("{:?} {:?} {}", a, b, dist);

    connects.get_mut(&a).unwrap().push(b);
    connects.get_mut(&b).unwrap().push(a);
    circuits = connect(&circuits, &a, &b)
  }

  circuits
    .iter()
    .map(|c| c.len() as u64)
    .sorted()
    .rev()
    .take(3)
    .product()
}

fn part_2(coords: &Vec<Coord>, pairs: &Pairs) -> u64 {
  let mut circuits: Vec<Circuit> = vec![];
  let mut connects: Connects = HashMap::new();
  for coord in coords.clone() {
    circuits.push(HashSet::from([coord]));
    connects.insert(coord, vec![]);
  }

  let mut last: Vec<Coord> = vec![];

  while circuits.len() > 1 {
    let (a, b, _dist) = pairs
      .iter()
      .map(|(k, v)| {
        let closest = v
          .iter()
          .find(|c| !&connects[k].contains(&c.0))
          .unwrap();
        (*k, closest.0, closest.1)
      })
      .min_by(|x, y| x.2.partial_cmp(&y.2).unwrap())
      .unwrap();

    // println!("{:?} {:?} {}", a, b, dist);
    
    connects.get_mut(&a).unwrap().push(b);
    connects.get_mut(&b).unwrap().push(a);
    circuits = connect(&circuits, &a, &b);

    last = vec![a,b];
  }

  last.iter().map(|c| c.0 as u64).product()
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
  let pairs = get_closest_pairs(&coords);
  
  let part1: u64 = part_1(&coords, &pairs, 1000);
  let part2: u64 = part_2(&coords, &pairs);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
}

inventory::submit! { crate::Day { year: 2025, day: 8, run } }
