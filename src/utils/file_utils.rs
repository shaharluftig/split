use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

pub fn create_file_buffer(path: &PathBuf) -> BufWriter<File> {
    let mut file = match File::create(&path) {
        Err(err) => panic!("Couldn't write {file_name}:{reason}",
                           file_name = &path.file_name().and_then(OsStr::to_str).unwrap(), reason = err),
        Ok(file) => file,
    };
    BufWriter::new(file)
}

pub fn read_file(path: &PathBuf) -> File {
    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {file_name}:{reason}",
                           file_name = &path.file_name().and_then(OsStr::to_str).unwrap(), reason = err),
        Ok(file) => file,
    };
    file
}