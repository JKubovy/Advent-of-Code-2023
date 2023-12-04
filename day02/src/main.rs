const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn first_part(input: &str) -> u32 {
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

fn second_part(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let index_of_semicolon = line.find(':').unwrap();
            let mut max_red = 1u32;
            let mut max_green = 1u32;
            let mut max_blue = 1u32;
            line[index_of_semicolon + 2..].split("; ").for_each(|play| {
                play.split(", ").for_each(|cube| {
                    let (count, color) = cube.split_once(' ').unwrap();
                    let count: u32 = count.parse().unwrap();
                    match (count, color) {
                        (count, "red") if count > max_red => max_red = count,
                        (count, "green") if count > max_green => max_green = count,
                        (count, "blue") if count > max_blue => max_blue = count,
                        _ => {}
                    }
                });
            });
            max_red * max_green * max_blue
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day02.input");
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
        let data = include_str!("../../inputs/day02.test");
        let result = first_part(data);
        assert_eq!(result, 8);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../../inputs/day02.input");
        let result = first_part(data);
        assert_eq!(result, 2169);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../../inputs/day02.test");
        let result = second_part(data);
        assert_eq!(result, 2286);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../../inputs/day02.input");
        let result = second_part(data);
        assert_eq!(result, 60948);
    }
}
