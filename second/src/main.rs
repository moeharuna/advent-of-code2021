use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

fn lines_to_int(lines: Lines<BufReader<File>>) -> Vec<i32> {
    lines
        .map(|result_line| result_line.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let file = File::open("input.txt").expect("file input.txt not found");
    let lines = lines_to_int(read_lines(file));
    let mut last: i32 = lines.iter().take(3).sum();
    let mut counter: i32 = 0;
    let mut iter = lines.iter();
    //using for loop here caused some ownership issues
    loop {
        iter.next();
        let next_three = iter.clone().take(3);
        if next_three.len() != 3 {
            break;
        }
        let sum = next_three.sum();
        if last < sum {
            counter += 1;
        }
        last = sum;
    }
    println!("{}", counter);
}
