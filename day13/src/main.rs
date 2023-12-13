use std::cmp::min;

use grid::*;

#[derive(Clone, PartialEq)]
enum Ground {
    Ash,
    Rock,
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            '.' => Ground::Ash,
            '#' => Ground::Rock,
            _ => panic!("Unknown ground"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Grid<Ground>> {
    let (mut result, current, width) = input.lines().fold(
        (Vec::new(), Vec::new(), 0),
        |(mut result, mut current, width), line| {
            if line.is_empty() {
                result.push(Grid::from_vec(current, width));
                return (result, Vec::new(), 0);
            }
            let line = line.chars().map(|c| c.into()).collect::<Vec<_>>();
            let len = line.len();
            current.extend(line);
            (result, current, len)
        },
    );
    result.push(Grid::from_vec(current, width));
    result
}

fn number_of_errors_in_row(line: &Vec<&Ground>, index: usize) -> usize {
    let mut errors = 0;
    let width = line.len();
    let half = min(width - index, index);
    for i in 0..half {
        if line[index - i - 1] != line[index + i] {
            errors += 1;
        }
    }
    errors
}

fn calculate_mirror_index(input: &Grid<Ground>, allow_erors: usize) -> usize {
    let (height, width) = input.size();
    for split in 1..width {
        let mut number_of_errors = 0;
        for row in input.iter_rows() {
            let row = row.collect::<Vec<_>>();
            number_of_errors += number_of_errors_in_row(&row, split);
        }
        if number_of_errors == allow_erors {
            return split;
        }
    }
    for split in 1..height {
        let mut number_of_errors = 0;
        for column in input.iter_cols() {
            let column = column.collect::<Vec<_>>();
            number_of_errors += number_of_errors_in_row(&column, split);
        }
        if number_of_errors == allow_erors {
            return 100 * split;
        }
    }
    panic!("No splits found")
}

fn first_part(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|input| calculate_mirror_index(input, 0))
        .sum()
}

fn second_part(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|input| calculate_mirror_index(input, 1))
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
        assert_eq!(result, 405);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 33735);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 400);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 38063);
    }
}
