fn calculate_calibration_value(input: &str) -> u32 {
    let result = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|numbers| numbers[0] * 10 + numbers[numbers.len() - 1])
        .sum::<u32>();
    result
}

fn main() {
    let input = include_str!("../input.txt");
    let result = calculate_calibration_value(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = include_str!("../test.txt");
        let result = calculate_calibration_value(data);
        assert_eq!(result, 142);
    }
}
