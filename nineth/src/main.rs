use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_point(string: &str) -> Point {
    let mut split = string.split(",");
    let x = split.next().unwrap().trim().parse::<i32>().unwrap();
    let y = split.next().unwrap().trim().parse::<i32>().unwrap();
    Point { x, y }
}

fn get_input() -> Vec<(Point, Point)> {
    let lines = read_lines(File::open("input.txt").unwrap());

    lines
        .map(|l| {
            let l = l.unwrap();
            let mut points = l.split("->");
            (
                parse_point(points.next().unwrap()),
                parse_point(points.next().unwrap()),
            )
        })
        .collect()
}

fn main() {
    dbg!(get_input());
    println!("Hello, world!");
}
