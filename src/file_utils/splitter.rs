use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

pub fn split_file(path: &PathBuf, lines: usize, files: usize, ignore_empty_lines: bool) {
    let file = read_file(&path);
    let reader: BufReader<File> = BufReader::new(file);
    let mut file_index: usize = 1;
    let mut file_buffer: BufWriter<File> = create_file_buffer(&format_path(&path, &file_index));

    for (line_index, line) in reader.lines().enumerate() {
        let row_num: usize = &line_index + 1;
        let line_value: String = line.unwrap();
        if ignore_empty_lines && &line_value == "" {
            continue;
        }
        writeln!(file_buffer, "{}", &line_value).expect(format!("Unable to write line:{}", row_num).as_str());
        if row_num % &lines == 0 && row_num != 0 {
            file_index = &file_index + 1;
            let formatted_path: PathBuf = format_path(&path, &file_index);
            file_buffer = create_file_buffer(&formatted_path);
            println!("File:{} written successfully", &formatted_path.file_name().and_then(OsStr::to_str).unwrap());
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

fn create_file_buffer(path: &PathBuf) -> BufWriter<File> {
    let mut file = match File::create(&path) {
        Err(err) => panic!("Couldn't write {file_name}:{reason}",
                           file_name = &path.file_name().and_then(OsStr::to_str).unwrap(), reason = err),
        Ok(file) => file,
    };
    BufWriter::new(file)
}

fn read_file(path: &PathBuf) -> File {
    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open {file_name}:{reason}",
                           file_name = &path.file_name().and_then(OsStr::to_str).unwrap(), reason = err),
        Ok(file) => file,
    };
    file
}