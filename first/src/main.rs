use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

fn lines_to_string(lines: Lines<BufReader<File>>) -> Vec<String> {
    lines.map(|result_line| result_line.unwrap()).collect()
}

fn main() {
    let file = File::open("input.txt").expect("file input.txt not found");
    let lines = lines_to_string(read_lines(file));
    let mut last = lines[0].parse::<i32>().unwrap();
    let mut counter = 0;

    for i in lines.iter() {
        let value = i.parse::<i32>().unwrap();
        if value > last {
            counter += 1;
        }
        last = value;
    }
    println!("{}", counter);
}
