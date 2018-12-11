use std::collections::HashSet;
use std::iter::{repeat};

use crate::utils::{read_input};

fn read_input_file(path: &str) -> Vec<i32> {
  read_input(path).iter().enumerate().map(|(i, line): (usize, &String)|
    line.parse::<i32>().unwrap_or_else(|_| panic!(
      "Unable to parse line {}, '{}' to a number",
      i, line
    ))
  ).collect()
}

/// Seems to be a simple case of reading the input and summing it
pub fn solve_part1(path: &str) -> i32 {
  read_input_file(path).iter().sum()
}

/// This one is trickier, we need to do the summing but also keep track of current sum and all
/// the sums we've already encountered. We do it by having a state that keeps track of
/// current sum - acc, and a HashSet reference that tracks all the sums that were encountered
/// up to the point. Final trick is to use a short-circuting of Rust Scan to exit as soon as
/// we encounter acc value that is already present in a set.
pub fn solve_part2(path: &str) -> i32 {
  let numbers = read_input_file(path);
  let mut acc: i32 = 0;
  let mut set: HashSet<i32> = HashSet::new();

  repeat(numbers).flatten().scan((&mut acc, &mut set), |state, item| {
    *state.0 += item; // Add current item to generate new sum
    if state.1.contains(state.0) {
      // Woohoo! We've already encountered this sum - stop the iterator, the acc will now be set
      // to a sum that has already been seen once
      None
    } else {
      // Add a new sum to the set and continue iteration
      state.1.insert(*state.0);
      Some(())
    }
  })
  // That's a one way to force full evaluation of an iterator, probably not the most elegant one
  .count();
  
  // Once we reached this point, acc is set to a value we've seen twice. Alternatively, previous call
  // to count will just run an endless loop if no sum is ever encountered twice
  acc 
}
