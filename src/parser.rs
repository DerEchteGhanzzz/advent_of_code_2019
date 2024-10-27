use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn get_input(day: &str) -> Vec<String> {
    lines_from_file(format!("input_files/day_{day}.txt"))
}