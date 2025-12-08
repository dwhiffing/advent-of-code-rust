use std::collections::{HashMap, HashSet};

type Coord = (u64, u64);
type Graph = HashMap<Coord, Vec<Coord>>;

fn build_graph(lines: &Vec<String>) -> Graph {
  let mut graph: Graph = HashMap::new();

  for (_y, line) in lines.iter().enumerate() {
    for (_x, char) in line.chars().enumerate() {
      if _y + 1 > lines.len()  { continue; }

      let (x, y) = (_x as u64, _y as u64);

      if char == '^' {
        let left = (x - 1, y + 1);
        let right = (x + 1, y + 1);
        graph.insert((x, y), vec![left, right]);
      } else {
        let down = (x, y + 1);
        graph.insert((x, y), vec![down]);
      }
    }
  }

  graph
}

fn part_1(start_x: u64, graph: &Graph) -> u64 {
  let mut stack: Vec<Coord> = vec![(start_x, 1)];
  let mut visited: HashSet<Coord> = HashSet::new();
  let mut splits: HashSet<Coord> = HashSet::new();

  while let Some((x, y)) = stack.pop() {
    if !graph.contains_key(&(x, y)) { continue }

    let paths = graph.get(&(x, y)).unwrap();
    if paths.len() > 1 { splits.insert((x, y + 1)); }
    for &n in paths {
      if visited.insert(n) { stack.push(n); }
    }
  }

  splits.len() as u64
}

fn part_2(start_x: u64, graph: &Graph) -> u64 {
  fn count_paths(node: &Coord, graph: &Graph, cache: &mut HashMap<Coord, u64>) -> u64 {
    if cache.contains_key(node) { return *cache.get(node).unwrap() }
    
    let count = match graph.get(&node) {
      None => 1,
      Some(paths) => 
        paths.iter().map(|p| count_paths(p, graph, cache)).sum(),
    };
    cache.insert(*node, count);
    count
  }

  count_paths(&(start_x, 1), graph, &mut HashMap::new())
}

pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("aoc_2025/07");
  let start_x = lines[0].chars().position(|c| c == 'S').unwrap_or(0) as u64;
  let graph = build_graph(&lines);
  
  let part1: u64 = part_1(start_x, &graph);
  let part2: u64 = part_2(start_x, &graph);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
}

inventory::submit! { crate::Day { year: 2025, day: 7, run } }
