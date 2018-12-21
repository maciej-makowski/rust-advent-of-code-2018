use std::collections::{HashMap, HashSet};
use std::str;
use std::cmp::min;
use regex::Regex;

use crate::utils::read_input;

lazy_static! {
  static ref LINE_PARSE: Regex = Regex::new("^Step ([A-Z]{1}) must be finished before step ([A-Z]{1}) can begin.$")
    .unwrap();
}

fn build_graph(dependencies: &[(u8, u8)]) -> HashMap<u8, HashSet<u8>> {
  let mut result: HashMap<u8, HashSet<u8>> = HashMap::new();
  dependencies.iter().fold(&mut result, |acc, (before, after)| {
    let deps = acc.entry(*after).or_insert_with(HashSet::new);
    deps.insert(*before);

    if !acc.contains_key(before) {
      acc.insert(*before, HashSet::new());
    }

    acc
  });
  result
}

fn resolve_next_step(dependencies: &HashMap<u8, HashSet<u8>>, completed: &[u8]) -> Option<u8> {
  let mut doable: Vec<u8> = dependencies.iter().filter(|(_, deps)| 
    deps.iter().all(|d| completed.contains(d))
  ).map(|(name, _)| 
    name
  ).cloned().collect();

  doable.sort();
  doable.iter().cloned().nth(0)
}

fn resolve_order(dependencies: &mut HashMap<u8, HashSet<u8>>) -> String {
  let mut result: Vec<u8> = Vec::new();

  while !dependencies.is_empty() {
    let next_step = resolve_next_step(dependencies, &result)
      .unwrap_or_else(|| panic!("Unable to determine next step, result: {:?}, dependencies: {:?}", result, dependencies));

    dependencies.remove(&next_step);
    result.push(next_step);
  }

  String::from_utf8(result.iter().map(|c| c + b'A' - 1).collect()).unwrap()
}

fn resolve_time(dependencies: &mut HashMap<u8, HashSet<u8>>, offset: u32, workers: usize) -> u32 {
  let mut current_time = 0u32;
  let mut next_time: u32;
  let mut completed: Vec<u8> = Vec::new();
  let mut workers: Vec<Option<(u8, u32)>> = vec![None; workers];

  while !dependencies.is_empty() {    
    next_time = std::u32::MAX;

    for w in &mut workers {
      if let Some((task, completes_at)) = w {
        if *completes_at == current_time {
          completed.push(*task);
          *w = None;
        } else {
          next_time = min(next_time, *completes_at);
        }
      }
    }

    for w in &mut workers {
      if w.is_none() {
        let possible_next_step = resolve_next_step(dependencies, &completed);
        if let Some(next_step) = possible_next_step {
          dependencies.remove(&next_step);
          let completion_time = offset + u32::from(next_step) + current_time;
          next_time = min(next_time, completion_time);
          *w = Some((next_step, completion_time));
        }
      }
    }

    current_time = next_time;
  }

  current_time
} 

fn parse_dependency(line: &str) -> (u8, u8) {
  let matches = LINE_PARSE.captures(line).unwrap_or_else(|| panic!("Unable to parse line: {}", line));
 
  (
    matches.get(1).unwrap().as_str().as_bytes()[0] - b'A' + 1,
    matches.get(2).unwrap().as_str().as_bytes()[0] - b'A' + 1
  )
}

fn load_dependencues(path: &str) -> Vec<(u8, u8)> {
  read_input(path).iter().map(|line| parse_dependency(line)).collect()
}

pub fn solve_part1(path: &str) -> String {
  let dependencies = load_dependencues(path);
  let mut graph = build_graph(&dependencies);
  resolve_order(&mut graph)
}

pub fn solve_part2(path: &str) -> u32 {
  let dependencies = load_dependencues(path);
  let mut graph = build_graph(&dependencies);

  resolve_time(&mut graph, 60_u32, 5_usize)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_line_parsing() {
    assert_eq!(parse_dependency("Step A must be finished before step B can begin."), (1, 2))
  }
}
