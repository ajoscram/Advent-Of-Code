use std::{ path::Path, fs::File, io::{BufRead, BufReader}};
mod solver;

fn main() {
    let filename = format!("input/{}.txt", solver::DAY);
    solver::solve(read_lines(&filename));
}

fn read_lines(filename: &str) -> impl Iterator<Item = String> {
    let path = Path::new(filename);
    return match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => BufReader::new(file).lines().flatten(),
    };
}