use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new((first, second): (i32, i32)) -> Point {
        Point {
            x: first,
            y: second,
        }
    }
}

struct Submarine {
    aim: i32,
    position: Point,
}
impl Submarine {
    fn new() -> Submarine {
        let point = Point::new((0, 0));
        Submarine {
            aim: 0,
            position: point,
        }
    }
    fn execute_command(&mut self, command: Command) {
        use Command::*;
        match command {
            Forward(x) => {
                self.position.x += x;
                self.position.y += self.aim * x;
            }
            Up(aim) => self.aim += aim,
            Down(aim) => self.aim -= aim,
        }
    }
}

enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}
impl Command {
    fn new(direction: &str, amount: i32) -> Command {
        use Command::*;
        match direction {
            "forward" => Forward(amount),
            "up" => Up(amount),
            "down" => Down(amount),
            _ => panic!("Wrong direction input"),
        }
    }
}

fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

fn lines_to_commands(lines: Lines<BufReader<File>>) -> Vec<Command> {
    lines
        .map(|result_line| {
            let result_line = result_line.unwrap();
            let mut slice = result_line.splitn(2, ' ');
            let (first, second) = (
                slice.next().expect("wrong number of arguments in line"),
                slice.next().expect("wrong number of arguments in line"),
            );
            Command::new(first, second.parse().expect("Not a number"))
        })
        .collect()
}

fn main() {
    let commands = lines_to_commands(read_lines(File::open("input.txt").unwrap()));
    let mut submarine = Submarine::new();
    for c in commands {
        submarine.execute_command(c);
    }
    let (x, y) = (submarine.position.x, submarine.position.y);
    println!(
        "horiz is: {}, vert is: {}, result is: {}",
        x,
        y,
        i32::abs(x * y)
    );
}
