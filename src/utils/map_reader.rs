use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn map_reader(file_path: &str) -> Result<Vec<Vec<i32>>, std::io::Error> {
    let file: File;
    let reader: BufReader<File>;
    let mut result: Vec<Vec<i32>>;
    let mut line_vector: Vec<i32>;

    file = File::open(file_path)?;
    reader = BufReader::new(file);
    result = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        line_vector = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        result.push(line_vector);
    }
    Ok(result)
}
