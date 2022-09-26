use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

/// Create a new file and returns a read buffer to the file
/// # Arguments
/// * `path` - A path to the file
pub fn create_write_file_buffer(path: &PathBuf) -> BufWriter<File> {
    let mut file = match File::create(&path) {
        Err(err) => panic!("Couldn't write {file_name}:{reason}",
                           file_name = &path.file_name().and_then(OsStr::to_str).unwrap(), reason = err),
        Ok(file) => file,
    };
    println!("File:{} written successfully", &path.file_name().and_then(OsStr::to_str).unwrap());
    BufWriter::new(file)
}

/// Create a new file and returns a read buffer to the file
/// # Arguments
/// * `path` - A path to the file
pub fn create_read_file_buffer(path: &PathBuf) -> BufReader<File> {
    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {file_name}:{reason}",
                           file_name = &path.file_name().and_then(OsStr::to_str).unwrap(), reason = err),
        Ok(file) => file,
    };
    BufReader::new(file)
}