use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

pub fn split_file(path: &PathBuf, lines: usize, files: usize) {
    let reader = BufReader::new(File::open(path).expect("Cannot open file"));
    let mut file_index: usize = 1;
    let mut file_buffer = create_file_buffer(format_path(&path, file_index));

    for (line_index, line) in reader.lines().enumerate() {
        writeln!(file_buffer, "{}", line.unwrap()).expect("Unable to write line");
        if line_index % &lines == 0 && line_index != 0 {
            file_index = &file_index + 1;
            file_buffer = create_file_buffer(format_path(&path, file_index));
        }
    }
}

fn format_path(path: &PathBuf, file_number: usize) -> PathBuf {
    let path = format!("{file_name}_{index}.{file_stem}",
                       file_name = &path.file_stem().and_then(OsStr::to_str).unwrap(),
                       index = file_number.to_string(),
                       file_stem = &path.extension().and_then(OsStr::to_str).unwrap());
    println!("file:{} written successfully", path);
    PathBuf::from(path)
}

fn create_file_buffer(path: PathBuf) -> BufWriter<File> {
    let file = File::create(&path).expect("unable to create file");
    return BufWriter::new(file);
}