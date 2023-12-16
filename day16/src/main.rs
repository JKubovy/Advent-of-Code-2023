use std::ops::Add;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    MirrorLeft,  // /
    MirrorRight, // \
    SplitterHorizontal,
    SplitterVertical,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '/' => Tile::MirrorLeft,
            '\\' => Tile::MirrorRight,
            '-' => Tile::SplitterHorizontal,
            '|' => Tile::SplitterVertical,
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

impl Add<Direction> for Position {
    type Output = Option<Position>;

    fn add(self, rhs: Direction) -> Self::Output {
        let position = match rhs {
            Direction::Up => Position {
                y: self.y.checked_sub(1)?,
                ..self
            },
            Direction::Left => Position {
                x: self.x.checked_sub(1)?,
                ..self
            },
            Direction::Down => Position {
                y: self.y.checked_add(1)?,
                ..self
            },
            Direction::Right => Position {
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
                    (Tile::Empty, direction) => stack.push((position + direction, direction)),
                    (Tile::SplitterHorizontal, Direction::Left) => {
                        stack.push((position + Direction::Left, Direction::Left))
                    }
                    (Tile::SplitterHorizontal, Direction::Right) => {
                        stack.push((position + Direction::Right, Direction::Right))
                    }
                    (Tile::SplitterVertical, Direction::Up) => {
                        stack.push((position + Direction::Up, direction))
                    }
                    (Tile::SplitterVertical, Direction::Down) => {
                        stack.push((position + Direction::Down, direction))
                    }
                    (Tile::MirrorLeft, Direction::Up) => {
                        stack.push((position + Direction::Right, Direction::Right))
                    }
                    (Tile::MirrorLeft, Direction::Left) => {
                        stack.push((position + Direction::Down, Direction::Down))
                    }
                    (Tile::MirrorLeft, Direction::Down) => {
                        stack.push((position + Direction::Left, Direction::Left))
                    }
                    (Tile::MirrorLeft, Direction::Right) => {
                        stack.push((position + Direction::Up, Direction::Up))
                    }
                    (Tile::MirrorRight, Direction::Up) => {
                        stack.push((position + Direction::Left, Direction::Left))
                    }
                    (Tile::MirrorRight, Direction::Left) => {
                        stack.push((position + Direction::Up, Direction::Up))
                    }
                    (Tile::MirrorRight, Direction::Down) => {
                        stack.push((position + Direction::Right, Direction::Right))
                    }
                    (Tile::MirrorRight, Direction::Right) => {
                        stack.push((position + Direction::Down, Direction::Down))
                    }
                    (Tile::SplitterHorizontal, _) => {
                        stack.push((position.clone() + Direction::Left, Direction::Left));
                        stack.push((position + Direction::Right, Direction::Right))
                    }
                    (Tile::SplitterVertical, _) => {
                        stack.push((position.clone() + Direction::Up, Direction::Up));
                        stack.push((position + Direction::Down, Direction::Down))
                    }
                }
            }
        }
    }
    // for line in &energized {
    //     for x in line {
    //         print!("{}", if x.is_some() { '#' } else { '.' })
    //     }
    //     println!();
    // }
    // println!();

    energized
        .iter()
        .flat_map(|a| a.iter().map(|d| if d.is_some() { 1 } else { 0 }))
        .sum()
}

fn first_part(input: &str) -> usize {
    let map = parse_input(input);
    let (height, width) = (map.len(), map.get(0).expect("Empty puzzle").len());
    get_energized_count(
        &map,
        height,
        width,
        Position { y: 0, x: 0 },
        Direction::Right,
    )
}

fn second_part(input: &str) -> usize {
    let map = parse_input(input);
    let (height, width) = (map.len(), map.get(0).expect("Empty puzzle").len());
    let mut max = 0;
    for y in 0..height {
        let tmp_max =
            get_energized_count(&map, height, width, Position { y, x: 0 }, Direction::Right);
        max = std::cmp::max(max, tmp_max);
        let tmp_max = get_energized_count(
            &map,
            height,
            width,
            Position { y, x: width - 1 },
            Direction::Left,
        );
        max = std::cmp::max(max, tmp_max);
    }
    for x in 0..width {
        let tmp_max =
            get_energized_count(&map, height, width, Position { y: 0, x }, Direction::Down);
        max = std::cmp::max(max, tmp_max);
        let tmp_max = get_energized_count(
            &map,
            height,
            width,
            Position { y: height - 1, x },
            Direction::Up,
        );
        max = std::cmp::max(max, tmp_max);
    }
    max
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
