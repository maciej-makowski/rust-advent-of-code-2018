use crate::utils::read_input;

fn load_tree_list(path: &str) -> Vec<u32> {
  let lines = read_input(path);
  let input = lines.get(0)
    .unwrap_or_else(|| panic!("Unable to read line from input file: {}", path));

  parse_tree_list(input)
}

fn parse_tree_list(input_line: &str) -> Vec<u32> {
  input_line.split(' ').map(|s| 
    s.parse::<u32>().unwrap_or_else(|e| 
      panic!("Unable to convert to number: {:?}", e))
    ).collect()
}

fn parse_node(tree: &[u32], start_index: usize) -> (usize, u32) {
  let mut metadata_sum = 0u32;
  let children = tree[start_index];
  let metadata_entries = tree[start_index + 1] as usize;
  let mut next_index = start_index + 2;

  for _ in 0 .. children {
    let (end_index, child_sum) = parse_node(tree, next_index);
    metadata_sum += child_sum;
    next_index = end_index;
  }

  metadata_sum += &tree[next_index .. next_index + metadata_entries].iter().sum();
  
  (next_index + metadata_entries, metadata_sum)
}

pub fn solve_part1(path: &str) -> u32 {
  let tree = load_tree_list(path);
  parse_node(&tree, 0).1
}

pub fn solve_part2(_path: &str) -> u32 {
  panic!("Not implemented")
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