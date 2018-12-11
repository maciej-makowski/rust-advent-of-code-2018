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

/// Simple comparator that retruns true when there is exactly one
/// difference between ids
fn are_close(left_id: &str, right_id: &str) -> bool {
  if left_id.len() !=  right_id.len() {
    return false;
  }

  // TODO: Exit as soon as we found a second difference
  left_id.chars().zip(right_id.chars()).fold(0, |acc, (lc, rc)| {
    if lc == rc { acc } else { acc + 1} 
  }) == 1
}

/// O(n^2) iteration over all the ids that should return as soon
/// as the first match is found. Assuming there is a match, this
/// should never to more than (n over 2) iterations, since
/// 'are_close' comparision is comutative
pub fn solve_part2(path: &str) -> String {
  let input = read_input(path);
  let matched = input.iter().flat_map(|left_id|
    input.iter().map(move |right_id| (left_id, right_id))
  )
  .find(|pair| are_close(pair.0, pair.1))
  .unwrap_or_else(|| panic!("Couldn't find close pair"));

  matched.0.chars().zip(matched.1.chars())
    .filter(|(l, r)| l == r)
    .map(|t| t.0)
    .collect()
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn are_close_returns_false_when_strings_are_different_length() {
    assert!(!are_close("abcde", "abc"))
  }

  #[test]
  fn are_close_returns_false_when_strings_have_more_then_one_different_character() {
    assert!(!are_close("abcd", "abbb"))
  }

  #[test]
  fn are_close_returns_false_if_strings_are_same() {
    assert!(!are_close("abcd", "abcd"))
  }

  #[test]
  fn are_close_returns_true_if_strings_have_no_more_then_one_different_character() {
    assert!(are_close("abcd", "abcb"))
  }
}
