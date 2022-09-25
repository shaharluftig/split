use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn split_file(path: std::path::PathBuf, lines: usize, files: usize) {
    let reader = BufReader::new(File::open(path).expect("Cannot open file"));
    let mut line_index: usize = 0;
    let mut file_index: usize = 1;
    for line in reader.lines() {
        if line_index % &lines == 0 {
            println!("file:{}", file_index);
            file_index += 1
        }
        line_index += 1;
    }
}