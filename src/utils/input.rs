use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("Error opening file.");
    let reader = BufReader::new(file);
    let mut vec = Vec::new();
    for line in reader.lines() {
        vec.push(line.expect("Cannot read line."));
    }
    vec
}
