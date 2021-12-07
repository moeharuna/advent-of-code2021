use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

fn minmax(start: i32, end: i32) -> (i32, i32) {
    (min(start, end), max(start, end))
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

fn step_to(start: i32, finish: i32) -> i32 {
    if start > finish {
        return start - 1;
    }
    start + 1
}

impl Line {
    fn points_on_line(&self) -> Vec<Point> {
        if self.start.y == self.end.y {
            let (start, end) = minmax(self.start.x, self.end.x);
            (start..=end)
                .map(|x| Point { x, y: self.start.y })
                .collect()
        } else if self.start.x == self.end.x {
            let (start, end) = minmax(self.start.y, self.end.y);
            (start..=end)
                .map(|y| Point { x: self.start.x, y })
                .collect()
        } else {
            let mut x = self.start.x;
            let mut y = self.start.y;
            let mut result: Vec<Point> = Vec::new();
            while x != self.end.x {
                result.push(Point { x, y });
                x = step_to(x, self.end.x);
                y = step_to(y, self.end.y);
            }
            result.push(Point { x, y });
            result
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

fn main() {
    let lines: Vec<Line> = parse_input(get_input());
    let mut all_points: HashMap<Point, u32> = HashMap::new();

    dbg!(lines.len());

    for line in lines {
        let points = line.points_on_line();
        //        dbg!(points.len());
        for point in points {
            all_points
                .entry(point)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    dbg!(all_points.len());
    dbg!(Line {
        start: Point { x: 3, y: 1 },
        end: Point { x: 1, y: 3 }
    }
    .points_on_line());
    println!(
        "{}",
        all_points.values().filter(|value| **value >= 2).count()
    );
}
