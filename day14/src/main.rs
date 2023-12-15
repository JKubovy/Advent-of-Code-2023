#[derive(Clone)]
enum Ground {
    Round,
    Cube,
    Empty,
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            'O' => Ground::Round,
            '#' => Ground::Cube,
            '.' => Ground::Empty,
            _ => panic!("Unknown ground type"),
        }
    }
}

fn parse_map(input: &str) -> Vec<Vec<Ground>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().map(|c| c.into()).collect::<Vec<Ground>>());
        acc
    })
}

fn calculate_weight_for_part(height: usize, start_index: usize, round_count: usize) -> usize {
    let begin_number = height - start_index;

    (((begin_number + (begin_number - round_count + 1)) as f32 / 2f32) * round_count as f32)
        as usize
}

fn calculate_weight_to_north(map: Vec<Vec<Ground>>) -> usize {
    let (height, width) = (map.len(), map.get(0).expect("Empty puzzle").len());
    let mut result = 0;
    for x in 0..width {
        let mut start_index = 0;
        let mut round_count = 0;
        for y in 0..height {
            match map[y][x] {
                Ground::Round => round_count += 1,
                Ground::Cube => {
                    let weight = calculate_weight_for_part(height, start_index, round_count);
                    result += weight;
                    start_index = y + 1;
                    round_count = 0;
                }
                Ground::Empty => {}
            }
            if y == height - 1 && round_count > 0 {
                result += calculate_weight_for_part(height, start_index, round_count);
            }
        }
    }

    result
}

fn first_part(input: &str) -> usize {
    let map = parse_map(input);
    calculate_weight_to_north(map)
}

fn spin_map(map: Vec<Vec<Ground>>) -> Vec<Vec<Ground>> {
    let (height, width) = (map.len(), map.get(0).expect("Empty puzzle").len());
    let mut rotated_map = map.clone();

    todo!()
}

fn second_part(input: &str) -> usize {
    let map = parse_map(input);
    let map = spin_map(map);
    calculate_weight_to_north(map)
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
        assert_eq!(result, todo!());
    }
}
