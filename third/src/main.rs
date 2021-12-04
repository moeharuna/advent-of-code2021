use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

struct Command {
    direction: (i32, i32),
    amount: i32,
}

impl Command {
    fn convert_direction(direction: &str) -> (i32, i32) {
        match direction {
            "up" => (0, 1),
            "down" => (0, -1),
            "forward" => (1, 0),
            "backward" => (-1, 0),
            _ => panic!("Wrong direction input"),
        }
    }

    fn result(&self) -> (i32, i32) {
        let (first, second) = self.direction;
        (first * self.amount, second * self.amount)
    }

    fn new(direction: &str, amount: i32) -> Command {
        let direction = Command::convert_direction(direction);
        Command { direction, amount }
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
    let input = lines_to_commands(read_lines(File::open("input.txt").unwrap()));
    let (horiz, vert) = input
        .iter()
        .map(|command| command.result())
        .fold((0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));
    println!(
        "horiz is: {}, vert is: {}, result is: {}",
        horiz,
        vert,
        i32::abs(horiz * vert)
    );
}
