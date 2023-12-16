use std::{cmp::max, ops::Add};

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    MirrorLeft,  // /
    MirrorRight, // \
    SplitterHorizontal,
    SplitterVertical,
}

use Tile::*;

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            '/' => MirrorLeft,
            '\\' => MirrorRight,
            '-' => SplitterHorizontal,
            '|' => SplitterVertical,
            _ => panic!("Unknown tile type"),
        }
    }
}

#[derive(Clone)]
struct Position {
    y: usize,
    x: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

use Direction::*;

impl Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, rhs: Direction) -> Self::Output {
        let position = match rhs {
            Up => Position {
                y: self.y.checked_sub(1)?,
                ..self
            },
            Left => Position {
                x: self.x.checked_sub(1)?,
                ..self
            },
            Down => Position {
                y: self.y.checked_add(1)?,
                ..self
            },
            Right => Position {
                x: self.x.checked_add(1)?,
                ..self
            },
        };
        Some(position)
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect()
}

fn check_bound(position: Option<Position>, height: usize, width: usize) -> Option<Position> {
    if position.as_ref()?.y < height && position.as_ref()?.x < width {
        position
    } else {
        None
    }
}

fn get_energized_count(
    map: &[Vec<Tile>],
    height: usize,
    width: usize,
    start_position: Position,
    start_direction: Direction,
) -> usize {
    let mut energized: Vec<Vec<Option<Direction>>> = vec![vec![None; width]; height];
    let mut stack = vec![(Some(start_position), start_direction)];
    while let Some((position, direction)) = stack.pop() {
        if let Some(position) = check_bound(position, height, width) {
            let (y, x) = (position.y, position.x);
            let old_direction = energized[y][x];
            if old_direction.is_none() || old_direction.unwrap() != direction {
                energized[y][x] = Some(direction);
                match (map[y][x], direction) {
                    (Empty, direction) => stack.push((position + direction, direction)),
                    (SplitterHorizontal, Left) => stack.push((position + Left, Left)),
                    (SplitterHorizontal, Right) => stack.push((position + Right, Right)),
                    (SplitterVertical, Up) => stack.push((position + Up, direction)),
                    (SplitterVertical, Down) => stack.push((position + Down, direction)),
                    (MirrorLeft, Up) => stack.push((position + Right, Right)),
                    (MirrorLeft, Left) => stack.push((position + Down, Down)),
                    (MirrorLeft, Down) => stack.push((position + Left, Left)),
                    (MirrorLeft, Right) => stack.push((position + Up, Up)),
                    (MirrorRight, Up) => stack.push((position + Left, Left)),
                    (MirrorRight, Left) => stack.push((position + Up, Up)),
                    (MirrorRight, Down) => stack.push((position + Right, Right)),
                    (MirrorRight, Right) => stack.push((position + Down, Down)),
                    (SplitterHorizontal, _) => {
                        stack.push((position.clone() + Left, Left));
                        stack.push((position + Right, Right))
                    }
                    (SplitterVertical, _) => {
                        stack.push((position.clone() + Up, Up));
                        stack.push((position + Down, Down))
                    }
                }
            }
        }
    }

    energized
        .iter()
        .flat_map(|a| a.iter().map(|d| if d.is_some() { 1 } else { 0 }))
        .sum()
}

fn first_part(input: &str) -> usize {
    let map = parse_input(input);
    let (height, width) = (map.len(), map.get(0).expect("Empty puzzle").len());
    get_energized_count(&map, height, width, Position { y: 0, x: 0 }, Right)
}

fn second_part(input: &str) -> usize {
    let map = parse_input(input);
    let (height, width) = (map.len(), map.get(0).expect("Empty puzzle").len());
    let mut maximum = 0;
    for y in 0..height {
        let tmp_maximum = get_energized_count(&map, height, width, Position { y, x: 0 }, Right);
        maximum = max(maximum, tmp_maximum);
        let tmp_maximum =
            get_energized_count(&map, height, width, Position { y, x: width - 1 }, Left);
        maximum = max(maximum, tmp_maximum);
    }
    for x in 0..width {
        let tmp_maximum = get_energized_count(&map, height, width, Position { y: 0, x }, Down);
        maximum = max(maximum, tmp_maximum);
        let tmp_maximum =
            get_energized_count(&map, height, width, Position { y: height - 1, x }, Up);
        maximum = max(maximum, tmp_maximum);
    }
    maximum
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
        assert_eq!(result, 46);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 7788);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 51);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 7987);
    }
}
