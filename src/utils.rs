use std::io::{BufReader, BufRead};
use std::fs::{File};

pub fn read_input(path: &str) -> Vec<String> {
  let file: File = File::open(path).unwrap_or_else(|_| panic!("Unable to open file: {}", path));
  let reader: BufReader<&File> = BufReader::new(&file);

  reader
    .lines()
    .enumerate()
    .map(|(i, r_val)| r_val.unwrap_or_else(|_| panic!("Unable to read line {} from file {}", i, path)))
    .collect()
}