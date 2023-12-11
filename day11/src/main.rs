struct Position {
    x: usize,
    y: usize,
}

fn get_galaxy_position(input: &str) -> Vec<Position> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if let '#' = char {
                    return Some(Position { x, y });
                }
                None
            })
        })
        .collect()
}

fn expand_universe(mut galaxy_positions: Vec<Position>, expand_factor: usize) -> Vec<Position> {
    galaxy_positions.sort_by_key(|position| position.x);
    let mut x_expand = 0;
    let mut y_expand = 0;
    for i in 0..galaxy_positions.len() {
        let next_position = galaxy_positions.get(i + 1).unwrap_or(&galaxy_positions[i]);
        let x_expand_change = (next_position.x - galaxy_positions[i].x)
            .saturating_sub(1)
            .checked_mul(expand_factor)
            .expect("To large x_expand_change");
        galaxy_positions[i].x += x_expand;
        x_expand += x_expand_change;
    }

    galaxy_positions.sort_by_key(|position| position.y);
    for i in 0..galaxy_positions.len() {
        let next_position = galaxy_positions.get(i + 1).unwrap_or(&galaxy_positions[i]);
        let y_expand_change = (next_position.y - galaxy_positions[i].y)
            .saturating_sub(1)
            .checked_mul(expand_factor)
            .expect("To large y_expand_change");
        galaxy_positions[i].y += y_expand;
        y_expand += y_expand_change;
    }
    galaxy_positions.sort_by_key(|position| position.y);
    galaxy_positions
}

fn calculate_distances(galaxy_positions: &[Position]) -> Vec<usize> {
    let mut result = Vec::new();
    for first in 0..galaxy_positions.len() - 1 {
        for second in first + 1..galaxy_positions.len() {
            let first_position = &galaxy_positions[first];
            let second_position = &galaxy_positions[second];
            result.push(
                first_position.x.abs_diff(second_position.x)
                    + first_position.y.abs_diff(second_position.y),
            );
        }
    }
    result
}

fn first_part(input: &str) -> usize {
    let galaxy_positions = get_galaxy_position(input);
    let galaxy_positions = expand_universe(galaxy_positions, 1);
    calculate_distances(&galaxy_positions).iter().sum()
}

fn second_part(input: &str, expand_factor: usize) -> usize {
    let galaxy_positions = get_galaxy_position(input);
    let galaxy_positions = expand_universe(galaxy_positions, expand_factor - 1);
    calculate_distances(&galaxy_positions).iter().sum()
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let first_part = first_part(input);
    println!("First part: {}", first_part);
    let second_part = second_part(input, 1_000_000);
    println!("Second part: {}", second_part);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let data = include_str!("../inputs/test.txt");
        let result = first_part(data);
        assert_eq!(result, 374);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 10228230);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data, 10);
        assert_eq!(result, 1030);
        let result = second_part(data, 100);
        assert_eq!(result, 8410);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data, 1_000_000);
        assert_eq!(result, 447073334102);
    }
}
