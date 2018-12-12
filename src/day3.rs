use std::cmp::{max, min};
use std::collections::{HashSet};
use std::result::Result;

use regex::Regex;
use crate::utils::{read_input};

lazy_static! {
  static ref CLAIM_MATCHER: Regex = Regex::new(".?([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+).*")
    .unwrap();
}

#[derive(Debug)]
enum ClaimError {
  ParseLineError(String),
  ParseFieldError(String, String)
}

#[derive(Debug)]
struct Claim {
  id: i32,
  x: i32,
  y: i32,
  width: i32,
  height: i32
}

impl Claim {
  fn from_input(line: &str) -> Result<Claim, ClaimError> {
    let fields = vec!("id", "x", "y", "width", "height");
    let captures = CLAIM_MATCHER.captures(line)
      .ok_or_else(|| ClaimError::ParseLineError(line.to_string()))?;

    let matched_field_values: Result<Vec<i32>, ClaimError> = fields.iter()
      .zip(captures.iter()
        .skip(1)
        .map(|o| o.unwrap().as_str())
      )
      .map(|(field, str_value)| {
        str_value.parse::<i32>().or_else(|_| 
          Err(ClaimError::ParseFieldError(
            field.to_string(),
            str_value.to_string()
          )
        ))
      })
      .collect();

    let field_values = matched_field_values?;

    Ok(Claim {
      id: field_values[0],
      x: field_values[1],
      y: field_values[2],
      width: field_values[3],
      height: field_values[4]
    })
  }

  fn points(&self) -> HashSet<(usize, usize)> {
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    for i in self.x .. self.x + self.width {
      for j in self.y .. self.y + self.height {
        set.insert((i as usize, j as usize));
      }
    }
    set
  }

  fn overlaps_with(&self, claim: &Claim) -> bool {
    let ox = max(0, min(self.x + self.width, claim.x + claim.width) - max(self.x, claim.x));
    let oy = max(0, min(self.y + self.height, claim.y + claim.height) - max(self.y, claim.y));

    ox > 0 && oy > 0
  }
}

struct Fabric {
  points: Vec<usize>,
  width: usize
}

impl Fabric {
  fn new(width: usize, height: usize) -> Fabric {
    Fabric {
      points: vec![0; width * height],
      width: width
    }
  }

  fn point_index(&self, x: usize, y: usize) -> usize {
    x + (self.width * y)
  }

  pub fn inc_point(&mut self, x: usize, y: usize) {
    let i = self.point_index(x, y);
    self.points[i] += 1;
  }
}

fn load_claims(path: &str) -> Vec<Claim> {
  let lines = read_input(path);
  let parsed_claims: Result<Vec<Claim>, _> = lines.iter()
    .map(|s| Claim::from_input(s)).collect();
  parsed_claims.expect("An error occured while parsing claims")  
}

/// Not a big fan of how I solved this but I ran out of ideas. Basically, just tally up how many
/// times each point is claimed and then return number of points that are claimed more than once.
/// Minor efficiency gain by storing fabric in a flat array, not array of arrays, usually better
/// for CPU cache.
/// 
/// Still, I have a feeling there must be a better way
pub fn solve_part1(path: &str) -> usize {
  let claims = load_claims(path);
  let mut fabric = Fabric::new(1000, 1000);

  let fabric2 = claims.iter().fold(&mut fabric, |f, claim| {
    claim.points().iter().for_each(|p| f.inc_point(p.0, p.1));
    f
  });

  fabric2.points.iter().filter(|v| **v > 1).count()
}

/// Another O(n^2) solution where I go through entire search space.
/// Only improvement is an overlap implementation that doesn't have to go
/// through every single square.  In O(1) it calculates overlap area between 
/// 2 rectangles, so if it's zero, we know two claims don't overlap.
/// 
/// Also took me way to long to figure out that c1.overlaps_with(c2) is always
/// true when c1 == c2 😳
pub fn solve_part2(path: &str) -> i32 {
  let claims = load_claims(path);

  claims.iter()
    .find(|c1| !claims.iter().any(|c2| c1.id != c2.id && c1.overlaps_with(c2)))
    .expect("Can't find non-overlaping claim")
    .id
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn claim_should_parse_correct_input_line() {
    let claim = Claim::from_input(&"#123 @ 12,15: 10x25".to_string()).expect("Unable to parse claim");

    assert_eq!(claim.id, 123);
    assert_eq!(claim.x, 12);
    assert_eq!(claim.y, 15);
    assert_eq!(claim.width, 10);
    assert_eq!(claim.height, 25);
  }
}