use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

#[derive(Debug)]
struct BingoNumber {
    value: i32,
    is_drawn: bool,
}
#[derive(Debug)]
struct Board {
    content: Vec<BingoNumber>,
    width: usize,
    height: usize,
    bingo: Option<Vec<usize>>,
    did_bingo_before: bool,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, num) in self.content.iter().enumerate() {
            if num.is_drawn {
                write!(f, "\x1b[31m{:3}\x1b[0m", num.value)?;
            } else {
                write!(f, "{:3}", num.value)?;
            }
            if (i + 1) % self.width == 0 {
                write!(f, "\n")?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl Board {
    fn new(input: Vec<Vec<i32>>) -> Board {
        let height = input.len();
        let width = input[0].len();
        let content = input
            .iter()
            .flatten()
            .map(|num| BingoNumber {
                value: *num,
                is_drawn: false,
            })
            .collect();
        Board {
            content,
            width,
            height,
            bingo: None,
            did_bingo_before: false,
        }
    }

    fn get_number(&self, x: usize, y: usize) -> &BingoNumber {
        &self.content[y * self.width + x]
    }

    fn xy_from_index(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = (index - x) / self.width;
        (x, y)
    }

    fn index_from_xy(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn search_vertical_bingo(&self, position: (usize, usize)) -> Option<Vec<usize>> {
        let (x, y) = position;
        let vert_bingo: Vec<&BingoNumber> = (0..self.height)
            .map(|y| self.get_number(x, y))
            .filter(|num| num.is_drawn)
            .collect();
        if vert_bingo.len() < self.height {
            return None;
        }
        Some((0..self.height).map(|y| self.index_from_xy(x, y)).collect())
    }

    fn search_horiz_bingo(&self, position: (usize, usize)) -> Option<Vec<usize>> {
        let (x, y) = position;
        let horiz_bingo: Vec<&BingoNumber> = (0..self.width)
            .map(|x| self.get_number(x, y))
            .filter(|num| num.is_drawn)
            .collect();
        if horiz_bingo.len() < self.width {
            return None;
        }
        Some((0..self.width).map(|x| self.index_from_xy(x, y)).collect())
    }

    fn search_for_bingo(&self, index: usize) -> Option<Vec<usize>> {
        let position = self.xy_from_index(index);
        let vert = self.search_vertical_bingo(position);
        if let Some(_) = vert {
            return vert;
        }
        self.search_horiz_bingo(position)
    }
    fn draw_number_if_inside(&mut self, number: i32) -> bool {
        let mut drawned_tracker: Vec<usize> = Vec::new();
        for (i, num) in self.content.iter_mut().enumerate() {
            if num.value == number {
                drawned_tracker.push(i);
                num.is_drawn = true;
            }
        }
        if drawned_tracker.is_empty() {
            return false;
        }
        for i in drawned_tracker {
            if self.bingo.is_none() {
                self.bingo = self.search_for_bingo(i);
            }
        }
        true
    }

    fn bingo(&self) -> &Option<Vec<usize>> {
        &self.bingo
    }
}

fn get_input() -> (Vec<i32>, Vec<Board>) {
    let mut lines = read_lines(File::open("input.txt").unwrap());
    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
    let mut boards: Vec<Board> = vec![];
    let mut board_vec: Vec<Vec<i32>> = vec![];
    for l in lines {
        let l = l.unwrap();
        match l.trim() {
            "" if board_vec.len() > 0 => {
                boards.push(Board::new(board_vec));
                board_vec = vec![];
            }
            "" => (),
            _ => {
                let row: Vec<i32> = l
                    .split(' ')
                    .filter(|string| string.trim() != "")
                    .map(|string| string.parse::<i32>().unwrap())
                    .collect();
                board_vec.push(row);
            }
        }
    }
    (numbers, boards)
}

fn main() {
    let (numbers, mut boards) = get_input();
    let mut last_result = 0;
    for (index, n) in numbers.iter().enumerate() {
        for board in boards.iter_mut() {
            if board.draw_number_if_inside(*n) {
                if board.bingo().is_some() && !board.did_bingo_before {
                    let res: i32 = board
                        .content
                        .iter()
                        .filter(|num| !num.is_drawn)
                        .map(|num| num.value)
                        .sum::<i32>()
                        * n;
                    dbg!(index);
                    println!("number: {}. Bingo for board \n{}", n, board);
                    println!("result is:{}\n\n", res);
                    last_result = res;
                    board.did_bingo_before = true;
                }
            }
        }
    }
    dbg!(numbers.len());
    println!("Last result: {}", last_result);
}
