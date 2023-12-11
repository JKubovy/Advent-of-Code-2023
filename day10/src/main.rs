use std::{
    fmt::{Debug, Display},
    ops::Add,
};

#[derive(PartialEq)]
enum Pipe {
    None,       // .
    Start,      // S
    Vertical,   // │
    Horizontal, // ─
    DownRight,  // ┌ F
    DownLeft,   // ┐ 7
    UpLeft,     // ┘ J
    UpRight,    // └ L
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pipe::None => "·",
            Pipe::Start => "S",
            Pipe::Vertical => "│",
            Pipe::Horizontal => "─",
            Pipe::DownRight => "┌",
            Pipe::DownLeft => "┐",
            Pipe::UpLeft => "┘",
            Pipe::UpRight => "└",
        };
        write!(f, "{}", c)
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '.' => Pipe::None,
            'S' => Pipe::Start,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::UpRight,
            'J' => Pipe::UpLeft,
            '7' => Pipe::DownLeft,
            'F' => Pipe::DownRight,
            c => panic!("Unknown pipe type '{}'", c),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl From<Direction> for (isize, isize) {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}
impl Add<Direction> for &Position {
    type Output = Option<Position>;

    fn add(self, rhs: Direction) -> Self::Output {
        let (dir_y, dir_x) = rhs.into();
        let x = self.x.checked_add_signed(dir_x)?;
        let y = self.y.checked_add_signed(dir_y)?;
        Some(Position { x, y })
    }
}

fn get_new_direction(pipe: &Pipe, old_direction: Direction) -> Option<Direction> {
    match (old_direction, pipe) {
        (Direction::Up, Pipe::DownLeft) => Some(Direction::Left),
        (Direction::Up, Pipe::Vertical) => Some(Direction::Up),
        (Direction::Up, Pipe::DownRight) => Some(Direction::Right),
        (Direction::Left, Pipe::UpRight) => Some(Direction::Up),
        (Direction::Left, Pipe::Horizontal) => Some(Direction::Left),
        (Direction::Left, Pipe::DownRight) => Some(Direction::Down),
        (Direction::Down, Pipe::UpLeft) => Some(Direction::Left),
        (Direction::Down, Pipe::Vertical) => Some(Direction::Down),
        (Direction::Down, Pipe::UpRight) => Some(Direction::Right),
        (Direction::Right, Pipe::UpLeft) => Some(Direction::Up),
        (Direction::Right, Pipe::Horizontal) => Some(Direction::Right),
        (Direction::Right, Pipe::DownLeft) => Some(Direction::Down),
        _ => None,
    }
}

struct Board {
    pipes: Vec<Vec<Pipe>>,
    start: Position,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.pipes {
            for pipe in line {
                write!(f, "{}", pipe)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn get_start_direction(board: &Board) -> Direction {
    let direction = Direction::Up;
    let position = &board.start + direction;
    if let Some(position) = position {
        if let Pipe::DownLeft | Pipe::Vertical | Pipe::DownRight =
            board.pipes[position.y][position.x]
        {
            return direction;
        }
    }

    let direction = Direction::Left;
    let position = &board.start + direction;
    if let Some(position) = position {
        if let Pipe::UpRight | Pipe::Horizontal | Pipe::DownRight =
            board.pipes[position.y][position.x]
        {
            return direction;
        }
    }

    let direction = Direction::Down;
    let position = &board.start + direction;
    if let Some(position) = position {
        if let Pipe::UpLeft | Pipe::Vertical | Pipe::UpRight = board.pipes[position.y][position.x] {
            return direction;
        }
    }

    let direction = Direction::Right;
    let position = &board.start + direction;
    if let Some(position) = position {
        if let Pipe::UpLeft | Pipe::Horizontal | Pipe::DownLeft =
            board.pipes[position.y][position.x]
        {
            return direction;
        }
    }

    panic!("Unable to get start direction");
}

fn first_part(input: &str) -> usize {
    let board = get_board_with_start(input);
    let mut direction = get_start_direction(&board);
    let mut position = board.start;
    let mut steps = 0usize;
    let max_steps = board.pipes.len() * board.pipes[0].len();
    loop {
        steps += 1;
        position = (&position + direction).unwrap();
        let pipe = &board.pipes[position.y][position.x];
        if pipe == &Pipe::Start {
            break;
        }
        direction = get_new_direction(pipe, direction).expect("Can't get next direction");
        if steps >= max_steps {
            panic!("Loop too large")
        }
    }
    steps.div_ceil(2)
}

#[derive(Clone, PartialEq)]
enum Tile {
    Unknown,
    Inside,
    Outside,
    Pipe,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "!"),
            Self::Inside => write!(f, "I"),
            Self::Outside => write!(f, "."),
            Self::Pipe => write!(f, "-"),
        }
    }
}

fn second_part(input: &str) -> usize {
    let mut board = get_board_with_start(input);
    let mut tile_map = vec![vec![Tile::Unknown; board.pipes[0].len()]; board.pipes.len()];
    let start_direction = get_start_direction(&board);
    let mut direction = start_direction;
    let mut position = board.start.clone();
    let mut steps = 0usize;
    let max_steps = board.pipes.len() * board.pipes[0].len();
    loop {
        steps += 1;
        position = (&position + direction).unwrap();
        let pipe = &board.pipes[position.y][position.x];
        tile_map[position.y][position.x] = Tile::Pipe;
        if pipe == &Pipe::Start {
            break;
        }
        direction = get_new_direction(pipe, direction).expect("Can't get next direction");
        if steps >= max_steps {
            panic!("Loop too large")
        }
    }

    board.pipes[board.start.y][board.start.x] = match (start_direction, direction) {
        (Direction::Up, Direction::Up) => Pipe::Vertical,
        (Direction::Up, Direction::Left) => Pipe::UpRight,
        (Direction::Up, Direction::Right) => Pipe::UpLeft,
        (Direction::Left, Direction::Up) => Pipe::DownLeft,
        (Direction::Left, Direction::Left) => Pipe::Horizontal,
        (Direction::Left, Direction::Down) => Pipe::UpLeft,
        (Direction::Down, Direction::Left) => Pipe::DownRight,
        (Direction::Down, Direction::Down) => Pipe::Vertical,
        (Direction::Down, Direction::Right) => Pipe::DownLeft,
        (Direction::Right, Direction::Up) => Pipe::DownRight,
        (Direction::Right, Direction::Down) => Pipe::UpRight,
        (Direction::Right, Direction::Right) => Pipe::Horizontal,
        _ => panic!("Imposible"),
    };

    for (y, line) in board.pipes.iter_mut().enumerate() {
        (0..line.len()).for_each(|x| {
            if let Tile::Unknown = tile_map[y][x] {
                line[x] = Pipe::None;
            }
        })
    }

    for (y, line) in tile_map.iter_mut().enumerate() {
        let mut odd_number_pipes = false;
        (0..line.len()).for_each(|x| {
            (line[x], odd_number_pipes) = match (&board.pipes[y][x], odd_number_pipes) {
                (Pipe::None, true) => (Tile::Inside, true),
                (Pipe::None, false) => (Tile::Outside, false),
                (Pipe::Vertical, odd) => (Tile::Pipe, !odd),
                (Pipe::UpLeft, odd) => (Tile::Pipe, !odd),
                (Pipe::UpRight, odd) => (Tile::Pipe, !odd),
                (_, odd) => (Tile::Pipe, odd),
            }
        });
    }

    tile_map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, tile)| match (tile, &board.pipes[y][x]) {
                    (Tile::Inside, Pipe::None) => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn get_board_with_start(input: &str) -> Board {
    let mut start = None;
    let pipes = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let pipe = c.into();
                    if let Pipe::Start = pipe {
                        start = Some((x, y));
                    }
                    pipe
                })
                .collect()
        })
        .collect();
    let start = start.expect("Unable to find start position");
    Board {
        pipes,
        start: Position {
            x: start.0,
            y: start.1,
        },
    }
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
        assert_eq!(result, 8);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 6714);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test_second.txt");
        let result = second_part(data);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_second_part_custom_1() {
        let data = include_str!("../inputs/test_second_1.txt");
        let result = second_part(data);
        assert_eq!(result, 4);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_ne!(result, 60);
        assert_ne!(result, 16);
        assert!(result > 295);
        assert_eq!(result, 429);
    }

    #[test]
    fn test_add_position_and_direction() {
        let position = Position { x: 5, y: 5 };
        let new_position = (&position + Direction::Up).unwrap();
        assert_eq!(new_position.x, position.x);
        assert_eq!(new_position.y, 4);
        let new_position = (&position + Direction::Down).unwrap();
        assert_eq!(new_position.x, position.x);
        assert_eq!(new_position.y, 6);
        let new_position = (&position + Direction::Left).unwrap();
        assert_eq!(new_position.x, 4);
        assert_eq!(new_position.y, position.y);
        let new_position = (&position + Direction::Right).unwrap();
        assert_eq!(new_position.x, 6);
        assert_eq!(new_position.y, position.y);
    }
}
