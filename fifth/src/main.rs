use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

fn set_bit(number: u16, place: usize) -> u16 {
    1 << place | number
}

fn read_bit(number: u16, place: usize) -> u16 {
    number >> place & 1
}

fn input_parser(input: String) -> u16 {
    let input: String = input.chars().rev().collect();
    let mut result: u16 = 0;
    for (position, bit) in input.as_bytes().iter().enumerate() {
        if *bit == '1' as u8 {
            result = set_bit(result, position)
        }
    }

    result
}

fn gamma(input: Vec<u16>) -> u16 {
    let mut gamma: u16 = 0;
    let mut bit_counter = [0usize; 12];
    for i in &input {
        for bit_pos in 0..12 {
            if read_bit(*i, bit_pos) == 1 {
                bit_counter[bit_pos] += 1;
            }
        }
    }
    for (pos, count) in bit_counter.iter().enumerate() {
        if *count > (input.len() / 2) {
            gamma = set_bit(gamma, pos)
        }
    }
    gamma
}

fn read_bytes(f: File) -> Vec<u16> {
    read_lines(f)
        .map(|line| input_parser(line.unwrap()))
        .collect()
}

fn print_bits(value: u16) {
    println!("{:#014b}", value)
}

fn main() {
    let bytes = read_bytes(File::open("input.txt").unwrap());
    let gamma = gamma(bytes);
    let epsilon = (!gamma) << 16 - 12 >> 16 - 12; //quick and dirty way to fill max 4 bits with zeros
    println!("result = {}", gamma as i32 * epsilon as i32);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_parsing() {
        let mut line = read_lines(File::open("input.txt").unwrap());
        let string = line.next().unwrap().unwrap();
        assert_eq!(
            String::from("0b") + &string,
            format!("{:#014b}", input_parser(string))
        );
    }
}
