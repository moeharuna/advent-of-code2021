use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn points_on_line(&self) -> Vec<Point> {
        if self.start.y == self.end.y {
            let start = min(self.start.x, self.end.x);
            let end = max(self.start.x, self.end.x);
            (start..=end)
                .map(|x| Point { x, y: self.start.y })
                .collect()
        } else {
            let start = min(self.start.y, self.end.y);
            let end = max(self.start.y, self.end.y);
            (start..=end)
                .map(|y| Point { x: self.start.x, y })
                .collect()
        }
    }
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

fn parse_input(input: Vec<(Point, Point)>) -> Vec<Line> {
    input
        .into_iter()
        .map(|(start, end)| Line { start, end })
        .collect()
}

fn is_straight(line: &Line) -> bool {
    line.start.x == line.end.x || line.start.y == line.end.y
}

fn main() {
    let lines: Vec<Line> = parse_input(get_input())
        .into_iter()
        .filter(|line| is_straight(line))
        .collect();
    let mut all_points: HashMap<Point, u32> = HashMap::new();

    for line in lines {
        let points = line.points_on_line();
        for point in points {
            all_points
                .entry(point)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    dbg!(all_points.len());
    println!(
        "{}",
        all_points.values().filter(|value| **value >= 2).count()
    );
}
