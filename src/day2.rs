use std::collections::HashMap;

use crate::utils::{read_input};

/// Simply, for every box_id count the number of letters, then return
/// a tuple that indicates if it belongs to 2-letter and 3-letter
/// category 
fn categorize(box_id: &str) -> (i32, i32) {
  let mut state: HashMap<char, i32> = HashMap::new();

  box_id.chars().fold(&mut state, |s, c| {
    let current = s.entry(c).or_insert(0);
    *current += 1;
    s
  });

  match (state.values().any(|v| *v == 2), state.values().any(|v| *v == 3)) {
    (true, true) => (1, 1),
    (true, false) => (1, 0),
    (false, true) => (0, 1),
    _ => (0, 0)
  }
}

/// Fairly straightforward, categorize every id, sum the results and
/// multiply values in the result tuple - that should be the
/// checksum
pub fn solve_part1(path: &str) -> i32 {
  let checksum_components: (i32, i32) = read_input(path).iter()
    .map(|line| categorize(&line))
    .fold((0, 0), |acc, i| (acc.0 + i.0, acc.1 + i.1));

  checksum_components.0 * checksum_components.1
}