const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn calculate_calibration_value(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let index_of_semicolon = line.find(':').unwrap();
            let game_number: u32 = line[5..index_of_semicolon].parse::<u32>().unwrap();
            for play in line[index_of_semicolon + 2..].split("; ") {
                for cube in play.split(", ") {
                    let (count, color) = cube.split_once(' ').unwrap();
                    let count: u32 = count.parse().unwrap();
                    match (count, color) {
                        (count, "red") if count <= MAX_RED => continue,
                        (count, "green") if count <= MAX_GREEN => continue,
                        (count, "blue") if count <= MAX_BLUE => continue,
                        _ => return None,
                    }
                }
            }
            Some(game_number)
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day02.input");
    let result = calculate_calibration_value(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = include_str!("../../inputs/day02.test");
        let result = calculate_calibration_value(data);
        assert_eq!(result, 8);
    }
}
