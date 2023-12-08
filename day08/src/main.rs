use std::collections::HashMap;

const START: &str = "AAA";
const FINISH: &str = "ZZZ";

enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction {}", value),
        }
    }
}

fn first_part(input: &str) -> u32 {
    let (directions, left_instructions, right_instructions, _) = parse_input(input);
    let mut current_step = START;
    let mut directions = directions.into_iter().cycle().enumerate();
    while current_step != FINISH {
        let direction = directions.next().unwrap();
        current_step = make_step(
            current_step,
            direction.1.into(),
            &left_instructions,
            &right_instructions,
        );
    }
    directions.next().unwrap().0 as u32
}

fn make_step<'a>(
    begin: &'a str,
    direction: Direction,
    left: &'a HashMap<&'a str, &'a str>,
    right: &'a HashMap<&'a str, &'a str>,
) -> &'a str {
    match direction {
        Direction::Left => left.get(begin).unwrap(),
        Direction::Right => right.get(begin).unwrap(),
    }
}

fn parse_input(
    input: &str,
) -> (
    std::str::Chars<'_>,
    HashMap<&str, &str>,
    HashMap<&str, &str>,
    Vec<&str>,
) {
    let mut lines = input.lines();
    let directions = lines.next().expect("No directions").chars();
    _ = lines.next();
    let mut left_instructions = HashMap::new();
    let mut right_instructions = HashMap::new();
    let mut ghost_start = Vec::new();
    for instruction in lines {
        let mut tokens = instruction.split_ascii_whitespace();
        let begin = tokens.next().unwrap();
        _ = tokens.next();
        let left = &tokens.next().unwrap()[1..4];
        let right = &tokens.next().unwrap()[..3];
        left_instructions.insert(begin, left);
        right_instructions.insert(begin, right);
        if begin.ends_with('A') {
            ghost_start.push(begin)
        }
    }
    (
        directions,
        left_instructions,
        right_instructions,
        ghost_start,
    )
}

fn second_part(input: &str) -> u64 {
    let (directions, left_instructions, right_instructions, starts) = parse_input(input);
    let mut shortest_paths = Vec::new();
    starts.iter().for_each(|begin| {
        let mut directions = directions.clone().cycle().enumerate();
        let mut location = *begin;
        while !location.ends_with('Z') {
            let direction = directions.next().unwrap();
            location = make_step(
                location,
                direction.1.into(),
                &left_instructions,
                &right_instructions,
            );
        }
        shortest_paths.push(directions.next().unwrap().0)
    });
    dbg!(&shortest_paths);
    shortest_paths
        .iter()
        .map(|n| *n as u64)
        .reduce(lcm)
        .unwrap()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / (gcd(a, b))
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
        assert_eq!(result, 6);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 19631);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test_second.txt");
        let result = second_part(data);
        assert_eq!(result, 6);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 21003205388413);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(8, 6), 2);
        assert_eq!(gcd(6000, 8000), 2000);
        assert_eq!(gcd(2, 3), 1);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(5, 15), 15);
        assert_eq!(lcm(15, 5), 15);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(18, 12), 36);
        assert_eq!(lcm(2, 3), 6);
    }
}
