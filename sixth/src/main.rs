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
fn most_significant_bit(input: &Vec<u16>, position: usize) -> u16 {
    let mut counter = 0;
    for i in input {
        counter += read_bit(*i, position) as usize;
    }
    dbg!(counter);
    if counter >= (input.len() / 2) {
        return 1;
    }
    0
}

fn least_significant_bit(input: &Vec<u16>, position: usize) -> u16 {
    if most_significant_bit(input, position) == 1 {
        0
    } else {
        1
    }
}

fn filter_bits_until_last<P>(input: Vec<u16>, position: i32, bit_predicate: P) -> u16
where
    P: Fn(&Vec<u16>, usize) -> u16,
{
    if input.len() == 1 || position == 1 {
        return input[0];
    }
    let predicate = bit_predicate(&input, position as usize);
    filter_bits_until_last(
        input
            .into_iter()
            .filter(|value| read_bit(*value, position as usize) == predicate)
            .collect(),
        position - 1,
        bit_predicate,
    )
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
    let first = filter_bits_until_last(bytes.clone(), 11, most_significant_bit);
    let second = filter_bits_until_last(bytes, 11, least_significant_bit);
    print_bits(first);
    print_bits(second);
    println!("{}", first as i32 * second as i32);
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
