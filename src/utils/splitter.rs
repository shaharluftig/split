use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::utils::file_utils;

pub fn split_file(path: &PathBuf, lines: usize, files: usize, ignore_empty_lines: bool) {
    let file_read_buffer: BufReader<File> = file_utils::create_read_file_buffer(&path);
    let mut file_index: usize = 1;
    let mut file_writer_buffer: BufWriter<File> = file_utils::create_write_file_buffer(
        &format_path(&path, &file_index));

    for (line_index, line) in file_read_buffer.lines().enumerate() {
        let line_num: usize = &line_index + 1;
        let line_value: String = line.unwrap();
        if ignore_empty_lines && &line_value == "" {
            continue;
        }
        writeln!(file_writer_buffer, "{}", &line_value).expect(format!("Unable to write line:{}", line_num).as_str());
        if line_num % &lines == 0 && line_num != 0 {
            file_index = &file_index + 1;
            let formatted_path: PathBuf = format_path(&path, &file_index);
            file_writer_buffer = file_utils::create_write_file_buffer(&formatted_path);

        }
    }
}

fn format_path(path: &PathBuf, file_number: &usize) -> PathBuf {
    let path = format!("{file_name}_{index}.{file_stem}",
                       file_name = &path.file_stem().and_then(OsStr::to_str).unwrap(),
                       index = file_number.to_string(),
                       file_stem = &path.extension().and_then(OsStr::to_str).unwrap());
    PathBuf::from(path)
}
