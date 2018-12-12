use std::collections::VecDeque;
use std::cmp::min;

use crate::utils::read_input;

pub fn solve_part1(path: &str) -> usize {
  let lines = read_input(path);
  let polymer = lines.get(0).expect("Unable to read polymer");

  reduce_polymer(polymer).len()
}

pub fn solve_part2(path: &str) -> usize {
  let lines = read_input(path);
  let polymer = lines.get(0).expect("Unable to read polymer");

  (b'A' .. b'Z')
    .map(char::from)
    .map(|unit|
      polymer.chars().filter(|c|
        !c.eq_ignore_ascii_case(&unit)
      ).collect::<String>()
    )
    .map(|cleaned_polymer| reduce_polymer(&cleaned_polymer).len())
    .min()
    .unwrap()
}

fn reduce_polymer(polymer: &str) -> String {
  if polymer.is_empty() {
    return String::new()
  }

  let mut buffer: VecDeque<char> = polymer.chars().collect();
  let mut current_index = 0;

  loop {
    let mby_next_char = buffer.get(current_index + 1);
    if mby_next_char.is_none() {
      break buffer.iter().collect::<String>()
    }
    let next_char: &char = mby_next_char.unwrap();
    let current_char: &char = &buffer[current_index];

    if *next_char != *current_char && next_char.to_ascii_lowercase() == current_char.to_ascii_lowercase() {
      // Reduction!
      buffer.remove(current_index + 1);
      buffer.remove(current_index);

      current_index = if current_index == 0 { 0 } else { current_index - 1};
    } else {
      current_index = min(current_index + 1, buffer.len() - 1);
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_reduce_polymer() {
    assert_eq!(reduce_polymer("aAc"), "c");
    assert_eq!(reduce_polymer("Aac"), "c");
    assert_eq!(reduce_polymer("caA"), "c");
    assert_eq!(reduce_polymer("cAa"), "c");
    assert_eq!(reduce_polymer("cAAa"), "cA");
    assert_eq!(reduce_polymer("aBbA"), "");
    assert_eq!(reduce_polymer("c"), "c");
    assert_eq!(reduce_polymer("aBAb"), "aBAb");
  }
}
