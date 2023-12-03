#[derive(Clone)]
enum Cell {
    Digit(char),
    Symbol(char),
    Dot,
    Edge,
}

struct NumberLocation {
    number: u32,
    line: usize,
    start: usize,
    end_inclusive: usize,
}

fn calculate_calibration_value(input: &str) -> u32 {
    let board = parse_board(input);
    let board = expand_borad_with_edge(board);
    let numbers = get_numbers_with_coordinates(&board);
    let numbers = get_gear_ratios(numbers, &board);
    numbers.iter().sum()
}

fn get_gear_ratios(numbers: Vec<NumberLocation>, board: &[Vec<Cell>]) -> Vec<u32> {
    let mut gears = Vec::new();
    let stars = board
        .iter()
        .enumerate()
        .flat_map(|(line_number, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, cell)| match cell {
                    Cell::Symbol('*') => Some((line_number, x)),
                    _ => None,
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>();
    for (line, x) in stars {
        let mut first_number_index: Option<usize> = None;
        for y_shift in -1..=1 {
            for x_shift in -1..=1 {
                let (line_number, x) = (
                    line.checked_add_signed(y_shift).unwrap(),
                    x.checked_add_signed(x_shift).unwrap(),
                );
                let cell: &Cell = &board[line_number][x];
                if let Cell::Digit(_) = cell {
                    let number_index = find_number_index(line_number, x, &numbers);
                    if let Some(index) = first_number_index {
                        if index == number_index {
                            continue;
                        }
                        gears.push(numbers[index].number * numbers[number_index].number);
                        break;
                    } else {
                        first_number_index = Some(number_index);
                    }
                }
            }
        }
    }
    gears
}

fn find_number_index(line: usize, x: usize, numbers: &[NumberLocation]) -> usize {
    numbers
        .iter()
        .position(|n| n.line == line && n.start <= x && n.end_inclusive >= x)
        .expect("Every digits need to belogs to number")
}

fn get_numbers_with_coordinates(board: &[Vec<Cell>]) -> Vec<NumberLocation> {
    let mut numbers = Vec::new();
    board.iter().enumerate().for_each(|(line_number, line)| {
        let mut start = None;
        let mut number = 0u32;
        for (x, cell) in line.iter().enumerate() {
            match cell {
                Cell::Digit(d) => {
                    number = number * 10 + d.to_digit(10).unwrap();
                    start = start.or(Some(x));
                }
                Cell::Dot | Cell::Symbol(_) | Cell::Edge if start.is_some() => {
                    numbers.push(NumberLocation {
                        number,
                        line: line_number,
                        start: start.unwrap(),
                        end_inclusive: x - 1,
                    });
                    start = None;
                    number = 0;
                }
                Cell::Dot | Cell::Symbol(_) | Cell::Edge => continue,
            }
        }
    });
    numbers
}

fn expand_borad_with_edge(board: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut new_board = Vec::with_capacity(board.capacity() + 2);
    let length = board.get(0).map(|line| line.len()).unwrap_or(0);
    new_board.push(vec![Cell::Edge; length + 2]);
    board
        .into_iter()
        .for_each(|line| new_board.push([vec![Cell::Edge], line, vec![Cell::Edge]].concat()));
    new_board.push(vec![Cell::Edge; length + 2]);
    new_board
}

fn parse_board(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    c if c.is_ascii_digit() => Cell::Digit(c),
                    '.' => Cell::Dot,
                    c => Cell::Symbol(c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("../../inputs/day03.input");
    let result = calculate_calibration_value(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = include_str!("../../inputs/day03.test");
        let result = calculate_calibration_value(data);
        assert_eq!(result, 467835);
    }

    #[test]
    fn test_input() {
        let data = include_str!("../../inputs/day03.input");
        let result = calculate_calibration_value(data);
        assert_eq!(result, 81463996);
    }
}
