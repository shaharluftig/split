#[cfg(test)]
/// Generic Splitter tests
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    use crate::splitter::generic_splitter::split_file;
    use crate::utils::file_utils;

    /// A generic split line test
    /// # Arguments
    /// * `path` - A PathBuf slice to the file
    /// * `number_of_lines`- Number of lines per output file
    /// * `ignore_empty_lines`- A bool that indicates if empty lines should be ignored
    fn test_sanity(path: PathBuf, number_of_lines: usize, ignore_empty_lines: bool) {
        let written_files: Vec<PathBuf> = split_file(&path, number_of_lines, ignore_empty_lines);
        for path in written_files.iter() {
            let file_read_buffer: BufReader<File> = file_utils::create_read_file_buffer(&path);
            let mut line_counter = 0;
            for _line in file_read_buffer.lines() {
                line_counter += 1;
            }
            assert_eq!(number_of_lines, line_counter);
            fs::remove_file(path).expect("File doesn't exist");
        }
    }

    #[test]
    /// Tests --ignore-empty-lines flag
    fn test_ignore_empty_lines() {
        let path: PathBuf = PathBuf::from("./src/tests/resources/test_empty_lines.txt");
        test_sanity(path, 3, true)
    }

    #[test]
    /// Tests -l flag
    fn test_split_lines() {
        let path: PathBuf = PathBuf::from("./src/tests/resources/test.txt");
        test_sanity(path, 3, false)
    }
}