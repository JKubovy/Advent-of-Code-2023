#[derive(Clone, Copy, Hash)]
enum Ground {
    Round,
    Cube,
    Empty,
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use Ground::*;

impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            'O' => Round,
            '#' => Cube,
            '.' => Empty,
            _ => panic!("Unknown ground type"),
        }
    }
}

fn get_hash(map: &[Vec<Ground>]) -> u64 {
    let mut hash = DefaultHasher::new();
    for line in map {
        line.hash(&mut hash);
    }
    hash.finish()
}

fn parse_map(input: &str) -> Vec<Vec<Ground>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().map(|c| c.into()).collect::<Vec<Ground>>());
        acc
    })
}

fn calculate_weight(map: &[Vec<Ground>]) -> usize {
    let height = map.len();
    map.iter()
        .enumerate()
        .map(|(line_number, line)| {
            (height - line_number) * line.iter().filter(|&&tile| matches!(tile, Round)).count()
        })
        .sum()
}

fn first_part(input: &str) -> usize {
    let mut map = parse_map(input);
    slide_north(&mut map);
    calculate_weight(&map)
}

// rotate 90 degrees clockwise: (x, y) -> (y, -x)
fn rotate_clockwise(map: &mut Vec<Vec<Ground>>) {
    let (height, width) = (map.len(), map.get(0).expect("Empty puzzle").len());
    let mut rotated = vec![vec![Empty; height]; width];
    for (y, line) in map.iter().enumerate() {
        for (x, ground) in line.iter().enumerate() {
            rotated[x][height - 1 - y] = *ground;
        }
    }
    *map = rotated;
}

fn spin_map(map: &mut Vec<Vec<Ground>>) {
    for _ in 0..4 {
        slide_north(map);
        rotate_clockwise(map);
    }
}

fn slide_north(map: &mut Vec<Vec<Ground>>) {
    let (height, width) = (map.len(), map.get(0).expect("Empty puzzle").len());
    for x in 0..width {
        let mut index = 0;
        for y in 0..height {
            match map[y][x] {
                Cube => index = y + 1,
                Round => {
                    map[y][x] = map[index][x];
                    map[index][x] = Round;
                    index += 1;
                }
                _ => {}
            }
        }
    }
}

fn second_part(input: &str) -> usize {
    let mut map = parse_map(input);
    let mut weights = Vec::new();
    let mut map_hashes = Vec::new();
    let mut hash = get_hash(&map);
    while !map_hashes.contains(&hash) {
        let weight = calculate_weight(&map);
        weights.push(weight);
        map_hashes.push(get_hash(&map));
        spin_map(&mut map);
        hash = get_hash(&map)
    }
    let start_index = map_hashes.iter().position(|&h| h == hash).unwrap();
    let cycle_len = map_hashes.len() - start_index;
    weights[((1_000_000_000 - start_index) % cycle_len) + start_index]
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
        assert_eq!(result, 136);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 105003);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 64);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 93742);
    }
}
