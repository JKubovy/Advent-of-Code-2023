fn calculate_calibration_value(input: &str) -> u32 {
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
        assert_eq!(result, 2286);
    }
}
