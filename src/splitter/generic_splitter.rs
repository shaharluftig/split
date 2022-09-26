use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::utils::file_utils;

/// Splits a file to multiply files
/// # Arguments
/// * `path` - A PathBuf slice to the file
/// * `lines`- Number of lines per output file
/// * `ignore_empty_lines`- A bool that indicates if empty lines should be ignored
pub fn split_file(path: &PathBuf, lines: usize, ignore_empty_lines: bool) {
    let file_read_buffer: BufReader<File> = file_utils::create_read_file_buffer(&path);
    let mut file_index: usize = 1;
    let mut file_writer_buffer: BufWriter<File> = file_utils::create_write_file_buffer(
        &format_path(&path, &file_index));

    let mut line_number: usize = 0;
    for (line_index, line) in file_read_buffer.lines().enumerate() {
        let line_value: String = line.expect(format!("Cant read line:{}", line_index).as_str());
        if ignore_empty_lines && &line_value == "" {
            continue;
        }
        if line_number % &lines == 0 && line_number != 0 {
            file_index = &file_index + 1;
            let formatted_path: PathBuf = format_path(&path, &file_index);
            file_writer_buffer = file_utils::create_write_file_buffer(&formatted_path);
        }
        writeln!(file_writer_buffer, "{}", &line_value).expect(format!("Unable to write line:{}", line_number).as_str());
        line_number = &line_number + 1;
    }
}

/// Formats a path + file number to a new path
/// # Arguments
/// * `path` - A PathBuf slice to the file
/// * `file_number`- The nubmer of the output file
fn format_path(path: &PathBuf, file_number: &usize) -> PathBuf {
    let path: String = format!("{file_name}_{index}.{file_stem}",
                               file_name = &path.file_stem().and_then(OsStr::to_str).unwrap(),
                               index = file_number.to_string(),
                               file_stem = &path.extension().and_then(OsStr::to_str).unwrap());
    PathBuf::from(path)
}
