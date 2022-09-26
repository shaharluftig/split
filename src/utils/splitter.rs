use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::utils::file_utils;

/// Splits a file to n lines or n files
/// # Arguments
/// * `path` - A PathBuf slice to the file
/// * `lines`- Number of lines per output file
/// * `ignore_empty_lines`- A bool that indicates if empty lines should be ignored
pub fn split_file(path: &PathBuf, lines: usize, _files: usize, ignore_empty_lines: bool) {
    let file_read_buffer: BufReader<File> = file_utils::create_read_file_buffer(&path);
    let mut file_index: usize = 1;
    let mut file_writer_buffer: BufWriter<File> = file_utils::create_write_file_buffer(
        &format_path(&path, &file_index));

    for (line_index, line) in file_read_buffer.lines().enumerate() {
        let line_value: String = line.expect(format!("Cant read line:{}", line_index).as_str());
        if ignore_empty_lines && &line_value == "" {
            continue;
        }
        if line_index % &lines == 0 && line_index != 0 {
            file_index = &file_index + 1;
            let formatted_path: PathBuf = format_path(&path, &file_index);
            file_writer_buffer = file_utils::create_write_file_buffer(&formatted_path);
        }
        writeln!(file_writer_buffer, "{}", &line_value).expect(format!("Unable to write line:{}", line_index).as_str());
    }
}

fn format_path(path: &PathBuf, file_number: &usize) -> PathBuf {
    let path = format!("{file_name}_{index}.{file_stem}",
                       file_name = &path.file_stem().and_then(OsStr::to_str).unwrap(),
                       index = file_number.to_string(),
                       file_stem = &path.extension().and_then(OsStr::to_str).unwrap());
    PathBuf::from(path)
}
