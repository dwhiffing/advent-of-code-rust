use std::{ops::Range};
use itertools::{Itertools};

type Coord = (i64, i64);
type Tiles = Vec<Vec<Range<i64>>>;

fn part_1(coords: &Vec<Coord>) -> i64 {
  let mut max = 0;
  for (i, a) in coords.iter().enumerate() {
    for b in coords[i+1..coords.len()].iter() {
      let w = (a.0 - b.0).abs()+1;
      let h = (a.1 - b.1).abs()+1;
      let area = h * w;
      if area > max { max = area; }
    }
  }
  max
}

fn range_overlap(range1: &Range<i64>, range2: &Range<i64>) -> bool {
  range1.start < range2.end && range2.start < range1.end
}

fn outline_shape(coords: &Vec<Coord>, height: usize) -> Tiles {
  let mut tiles: Tiles = vec![vec![]; height];

  for (i, a) in coords.iter().enumerate() {
    let b = if i == coords.len()-1 { coords[0] } else { coords[i+1] };

    if a.1 == b.1 {
      let range = a.0.min(b.0)..a.0.max(b.0)+1;
      tiles[a.1 as usize].push(range);
    } else {
      let n = (a.1 - b.1).abs();
      for j in 0..n {
        let m = if a.1 > b.1 { -1 } else { 1 };
        let y = (a.1 + (j * m)) as usize;
        if !tiles[y].iter().any(|_r| _r.contains(&a.0)) {
          tiles[y].push(a.0..a.0+1);
        }
      }
    }
  }

  tiles
}

fn flood(tiles: &mut Tiles, x: usize, y: usize) {
  let mut stack = vec![(x , y)];

  while stack.len() > 0 {
    let (x, y) = stack.pop().unwrap();
    
    let mut lx = x as i64;
    let mut rx = x as i64;

    if tiles[y].len() == 2 {
      let a = &tiles[y][0];
      let b = &tiles[y][1];
      lx = a.start.min(b.start)+1;
      rx = a.start.max(b.start)-1;
    } else {
      while !tiles[y].iter().any(|t| t.contains(&(rx+1))) { rx += 1; }
      while lx > 0 && !tiles[y].iter().any(|t| t.contains(&(lx-1))) { lx -= 1; }
    }

    let scanline = (lx as i64)..(rx as i64) + 1;
    let mut r1 = scanline.clone();
    for r2 in &tiles[y] {
      r1.start = r1.start.min(r2.start);
      r1.end = r1.end.max(r2.end);
    }
    tiles[y] = vec![r1];

    let mut expand = |_y: usize| {
      if tiles[_y].len() < 2 { return }

      let a = tiles[_y][0].end;
      let b = tiles[_y][1].start;
      let r2 = a.min(b)..a.max(b);
      
      if range_overlap(&scanline, &r2) {
        stack.push((r2.start as usize, _y))
      }
    };
    
    expand(y - 1); 
    expand(y + 1); 

  }
}

fn largest_square(coords: &Vec<Coord>, tiles: &Tiles) -> i64 {
  let mut max = 0;
  for (i, a) in coords.iter().enumerate() {
    for b in coords[i+1..coords.len()].iter() {
      let w = (a.0 - b.0).abs() + 1;
      let h = (a.1 - b.1).abs() + 1;
      let area = h * w;
      if area <= max { continue }

      let ax = a.0.min(b.0);
      let ay = a.1.min(b.1);
      let bx = a.0.max(b.0);
      let by = a.1.max(b.1);
      
      let is_valid = (ay..by).all(|y| {
        let mut x: i64 = ax;
        for range in &tiles[y as usize] {
          if range.start <= x && range.end > x {
            x = range.end;
            if x >= bx { return true; }
          }
        }
        false
      });
      
      if is_valid { max = area }
    }
  }
  max
}

fn part_2(coords: &Vec<Coord>) -> i64 {
  let width = coords.iter().map(|f| f.0).max().unwrap() + 3;
  let height = coords.iter().map(|f| f.1).max().unwrap() + 2;

  let mut tiles: Tiles = outline_shape(coords, height as usize);

  flood(&mut tiles, (width/2) as usize, (height/2) as usize);

  largest_square(coords, &tiles)
}

pub fn run() {
  let lines: Vec<String> = crate::utils::read_lines("aoc_2025/09");

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
  
  let part1: i64 = part_1(&coords);
  let part2: i64 = part_2(&coords);

  println!("Part 1: {}\nPart 2: {}", part1, part2);
}

inventory::submit! { crate::Day { year: 2025, day: 9, run } }
