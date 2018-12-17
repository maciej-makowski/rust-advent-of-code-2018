use std::cmp::{max};
use std::collections::{HashSet, HashMap};

use crate::utils::read_input;

trait Coord {
  fn get_x(&self) -> i32;
  fn get_y(&self) -> i32;
}

impl Coord for (i32, i32) {
  fn get_x(&self) -> i32 {
    self.0
  }

  fn get_y(&self) -> i32 {
    self.1
  }
}

struct Map2D<T> {
  width: i32,
  height: i32,
  area: Vec<T>
}

impl <T> Map2D<T> where T: Clone {
  fn for_points<U>(coords: &[U], default: T) -> Map2D<T> where U: Coord {
    let (width, height) = get_map_dimensions(coords);
    Map2D::new(width, height, default)
  }

  fn new(width: i32, height: i32, default: T) -> Map2D<T> {
    Map2D {
      width,
      height,
      area: vec![default; (width * height) as usize]
    }
  }

  fn point_coords(&self, coord: &Coord) -> Option<usize> {
    if coord.get_x() >= self.width || coord.get_y() >= self.height {
      None
    } else {
      Some ((coord.get_x() + coord.get_y() * self.width) as usize)
    }
  }

  fn get(&self, coord: &Coord) -> Option<T> {
    self.point_coords(coord).and_then(|c| self.area.get(c)).map(|c| c.clone())
  }

  fn set(&mut self, coord: &Coord, value: T) -> Option<()> {
    self.point_coords(coord).and_then(|c| {
      self.area[c] = value;
      Some(())
    })
  }
}

fn get_map_dimensions<T>(coords: &[T]) -> (i32, i32) 
  where T: Coord {
  coords.iter().fold((0, 0), |acc, c| (
    max(acc.0, c.get_x()) + 1,
    max(acc.1, c.get_y()) + 1
  ))
}

#[derive(Debug)]
struct ParseError(String);

fn parse_point(line: &str) -> Result<(i32, i32), ParseError> {
  let parts: Vec<&str> = line.split(", ").collect::<Vec<&str>>();
  let x_str = parts.get(0).ok_or_else(|| ParseError(format!("Unable to parse X coordinates from {}", line)))?;
  let x = x_str.parse::<i32>().or_else(|e| Err(ParseError(format!("Unable to convert {} to number: {:?}", x_str,e))))?;

  let y_str = parts.get(1).ok_or_else(|| ParseError(format!("Unable to parse X coordinates from {}", line)))?;
  let y = y_str.parse::<i32>().or_else(|e| Err(ParseError(format!("Unable to convert {} to number: {:?}", x_str,e))))?;

  Ok((x, y))
}

#[derive(Clone, Debug)]
enum AreaSize {
  Infinite,
  Squares(i32)
}

#[derive(Clone, Debug, PartialEq)]
enum MapPoint {
  Unvisited,
  ClosestTo { owner: (i32, i32), distance: i32 },
  Contested
}

pub fn solve_part1(path: &str) -> i32 {
  let points = read_input(path).iter().map(|line|
    parse_point(line).unwrap_or_else(|err| panic!("Unable to parse points: {:?}", err))
  ).collect::<Vec<(i32, i32)>>();

  let mut current_distance = 0;
  let mut areas: HashMap<(i32, i32), AreaSize> = points.iter()
    .map(|p| (*p, AreaSize::Squares(1)))
    .collect();

  let mut map: Map2D<MapPoint> = Map2D::for_points(&points, MapPoint::Unvisited);
  points.iter().for_each(|p| {
    map.set(p, MapPoint::ClosestTo { owner: *p, distance: 0i32 });
  });


  let mut to_visit: Box<HashSet<(i32, i32)>> = Box::new(
    points.iter()
      .cloned()
      .collect()
  );

  loop {
    if to_visit.is_empty() {
      break;
    }

    let mut next_to_visit: Box<HashSet<(i32, i32)>> = Box::new(HashSet::with_capacity(to_visit.len() * 4));
    for p in to_visit.iter() {
      let map_value = map.get(p);
      
      if current_distance == 0 {
        next_to_visit.insert((p.get_x() - 1, p.get_y()));
        next_to_visit.insert((p.get_x() + 1, p.get_y()));
        next_to_visit.insert((p.get_x(), p.get_y() - 1));
        next_to_visit.insert((p.get_x(), p.get_y() + 1));
      } else if map_value.is_some() && map_value.unwrap() == MapPoint::Unvisited {
        let neighbours: Vec<((i32, i32), Option<MapPoint>)> = [
          (p.get_x() - 1, p.get_y()),
          (p.get_x() + 1, p.get_y()),
          (p.get_x(), p.get_y() - 1),
          (p.get_x(), p.get_y() + 1)
        ].iter().map(|c| (*c, map.get(c))).collect();

        let path: Vec<&MapPoint> = neighbours.iter().map(|n| &n.1)
          .filter(|p| {
            if let Some(MapPoint::ClosestTo { distance, .. }) = *p {
              *distance == current_distance - 1
            } else {
              false
            }
          })
          .map(|np| (np.as_ref().unwrap()))
          .collect();

        let on_edge = neighbours.iter().map(|n| &n.1).any(|np| np.is_none());
        let only_contested_neighbours = path.is_empty() && neighbours.iter().any(|p|
          p.1 == Some(MapPoint::Contested)
        );

        if path.len() > 1 {
          map.set(p, MapPoint::Contested);
        } else if let Some(MapPoint::ClosestTo { owner, .. }) = path.first() {
          areas.entry(*owner).and_modify(|v| {
            if let AreaSize::Squares(size) = *v {
              *v = if on_edge {
                AreaSize::Infinite
              } else {
                AreaSize::Squares(size + 1)
              }
            }
          });
          map.set(p, MapPoint::ClosestTo { owner: *owner, distance: current_distance});
        } else if only_contested_neighbours {
          map.set(p, MapPoint::Contested);
        } else if path.is_empty() {
          panic!("We shouldn't be here! Failed for {:?}, iteration: {:?}", p, current_distance);
        }

        neighbours.iter().filter(|n|
            n.1.is_some() && *n.1.as_ref().unwrap() == MapPoint::Unvisited
        ).for_each(|np| { next_to_visit.insert(np.0); });
      }
    }

    to_visit = next_to_visit;
    current_distance += 1;
  }


  let c = map.area.iter().filter(move |i| **i == MapPoint::Unvisited).count();
  println!("Unvisited: {:?}", c);
  println!("Areas: {:?}", areas);

  areas.values().map(|n| {
    match *n {
      AreaSize::Squares(s) => s,
      _ => 0i32
    }
  }).max().unwrap()
}

pub fn solve_part2(path: &str) -> i32 {
  panic!("Not implemented")
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_map_dimension_calculations() {
    assert_eq!(get_map_dimensions::<(i32, i32)>(&[]), (0, 0));
    assert_eq!(get_map_dimensions(&[
      (0, 1),
      (0, 2),
      (0, 3),
      (0, 5),
      (1, 1),
      (131, 3)
    ]), (131, 5));
  }

  #[test]
  fn test_point_parsing() {
    assert_eq!(parse_point("0, 0").unwrap(), (0, 0));
    assert_eq!(parse_point("10, 20").unwrap(), (10, 20));
  }
}