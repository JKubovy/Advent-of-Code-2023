use std::{ops::Mul, str::Chars};

#[derive(Default)]
struct Lens<'a> {
    id: &'a str,
    number: usize,
}

fn calculate_hash(chars: Chars) -> usize {
    chars.fold(0, |acc, c| (acc + c as usize).mul(17) % 256)
}

fn first_part(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|part| calculate_hash(part.chars()))
        .sum()
}

fn second_part(input: &str) -> usize {
    let mut boxes = {
        let mut boxes = Vec::with_capacity(256);
        for _ in 0..256 {
            boxes.push(Vec::<Lens>::new());
        }
        boxes
    };
    input.trim().split(',').for_each(|part| {
        if part.ends_with('-') {
            let part = part.strip_suffix('-').unwrap();
            let hash = calculate_hash(part.chars());
            let index = boxes[hash].iter().position(|lens| lens.id == part);
            if let Some(index) = index {
                boxes.get_mut(hash).expect("Unkown box").remove(index);
            }
        } else {
            let (part, number) = part.split_once('=').expect("Wrong format");
            let number = number.parse().expect("Wrong number format");
            let hash = calculate_hash(part.chars());
            let index = boxes[hash].iter().position(|lens| lens.id == part);
            if let Some(index) = index {
                boxes
                    .get_mut(hash)
                    .expect("Unkown box")
                    .get_mut(index)
                    .unwrap()
                    .number = number;
            } else {
                boxes
                    .get_mut(hash)
                    .expect("Unkown box")
                    .push(Lens { id: part, number })
            }
        }
    });
    boxes
        .iter()
        .enumerate()
        .map(|(box_index, lenses)| {
            lenses.iter().enumerate().fold(0, |acc, (i, lens)| {
                acc + (box_index + 1) * (i + 1) * lens.number
            })
        })
        .sum()
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let first_part = first_part(input);
    println!("First part: {}", first_part);
    let second_part = second_part(input);
    println!("Second part: {}", second_part);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let data = include_str!("../inputs/test.txt");
        let result = first_part(data);
        assert_eq!(result, 1320);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 516804);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 145);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 231844);
    }
}
