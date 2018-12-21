use crate::utils::read_input;

fn load_tree_list(path: &str) -> Vec<usize> {
  let lines = read_input(path);
  let input = lines.get(0)
    .unwrap_or_else(|| panic!("Unable to read line from input file: {}", path));

  parse_tree_list(input)
}

fn parse_tree_list(input_line: &str) -> Vec<usize> {
  input_line.split(' ').map(|s| 
    s.parse::<usize>().unwrap_or_else(|e| 
      panic!("Unable to convert to number: {:?}", e))
    ).collect()
}

fn sum_node_metadata(tree: &[usize], start_index: usize) -> (usize, usize) {
  let mut metadata_sum = 0_usize;
  let children = tree[start_index];
  let metadata_entries = tree[start_index + 1];
  let mut next_index = start_index + 2;

  for _ in 0 .. children {
    let (end_index, child_sum) = sum_node_metadata(tree, next_index);
    metadata_sum += child_sum;
    next_index = end_index;
  }

  metadata_sum += &tree[next_index .. next_index + metadata_entries].iter().sum();
  
  (next_index + metadata_entries, metadata_sum)
}

fn calc_node_value(tree: &[usize], start_index: usize) -> (usize, usize) {
  let mut node_value = 0_usize;

  let children_count = tree[start_index];
  let metadata_count = tree[start_index + 1];
  let mut children_values: Vec<usize> = Vec::with_capacity(children_count);
  let mut next_index = start_index + 2;

  for _ in 0 .. children_count {
    let (end_index, children_value) = calc_node_value(tree, next_index);
    children_values.push(children_value);
    next_index = end_index;
  }

  if children_count == 0 {
    node_value += &tree[next_index .. next_index + metadata_count].iter().sum();
  } else {
    node_value += &tree[next_index .. next_index + metadata_count].iter().filter_map(|index| {
      children_values.get(*index - 1)
    }).sum()
  }

  (next_index + metadata_count, node_value)
}

pub fn solve_part1(path: &str) -> usize {
  let tree = load_tree_list(path);
  sum_node_metadata(&tree, 0).1
}

pub fn solve_part2(path: &str) -> usize {
  let tree = load_tree_list(path);
  calc_node_value(&tree, 0).1
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_input_parsing() {
    assert_eq!(
      parse_tree_list("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"),
      vec!(2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2)
    );
  }
}