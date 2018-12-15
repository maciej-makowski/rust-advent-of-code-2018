use std::cmp::{max};

use crate::utils::read_input;

trait Coord {
  fn get_x(&self) -> u32;
  fn get_y(&self) -> u32;
}

impl Coord for (u32, u32) {
  fn get_x(&self) -> u32 {
    self.0
  }

  fn get_y(&self) -> u32 {
    self.1
  }
}

struct Map2D<T> {
  width: u32,
  height: u32,
  area: Vec<T>
}

impl <T> Map2D<T> where T: Clone {
  fn for_points<U>(coords: &[U], default: T) -> Map2D<T> where U: Coord {
    let (width, height) = get_map_dimensions(coords);
    Map2D::new(width, height, default)
  }

  fn new(width: u32, height: u32, default: T) -> Map2D<T> {
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

  fn get(&self, coord: &Coord) -> Option<&T> {
    self.point_coords(coord).and_then(|c| self.area.get(c))
  }

  fn set(&mut self, coord: &Coord, value: T) -> Option<()> {
    self.point_coords(coord).and_then(|c| {
      self.area[c] = value;
      Some(())
    })
  }
}

fn get_map_dimensions<T>(coords: &[T]) -> (u32, u32) 
  where T: Coord {
  coords.iter().fold((0, 0), |acc, c| (
    max(acc.0, c.get_x()),
    max(acc.1, c.get_y())
  ))
}

#[derive(Debug)]
struct ParseError(String);

fn parse_point(line: &str) -> Result<(u32, u32), ParseError> {
  let parts: Vec<&str> = line.split(", ").collect::<Vec<&str>>();
  let x_str = parts.get(0).map(|v| *v).ok_or_else(|| ParseError(format!("Unable to parse X coordinates from {}", line)))?;
  let x = x_str.parse::<u32>().or_else(|e| Err(ParseError(format!("Unable to convert {} to number: {:?}", x_str,e))))?;

  let y_str = parts.get(1).map(|v| *v).ok_or_else(|| ParseError(format!("Unable to parse X coordinates from {}", line)))?;
  let y = y_str.parse::<u32>().or_else(|e| Err(ParseError(format!("Unable to convert {} to number: {:?}", x_str,e))))?;

  Ok((x, y))
}

pub fn solve_part1(path: &str) -> u32 {
  let points = read_input(path).iter().map(|line|
    parse_point(line).unwrap_or_else(|err| panic!("Unable to parse points: {:?}", err))
  ).collect::<Vec<(u32, u32)>>();

  let dims = get_map_dimensions(&points);
  panic!("Dims: {:?}", dims);
}

pub fn solve_part2(path: &str) -> u32 {
  panic!("Not implemented")
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_map_dimension_calculations() {
    assert_eq!(get_map_dimensions::<(u32, u32)>(&[]), (0, 0));
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