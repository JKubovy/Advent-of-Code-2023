fn calculate_calibration_value(input: &str) -> u32 {
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let result = input
        .lines()
        .map(|line| {
            let mut list: Vec<u32> = Vec::new();
            'iterate_chars: for (i, c) in line.chars().enumerate() {
                if let Some(n) = c.to_digit(10) {
                    list.push(n);
                    continue 'iterate_chars;
                }
                for (n, pattern) in patterns.iter().enumerate() {
                    if line[i..].starts_with(pattern) {
                        let n: u32 = (n + 1).try_into().unwrap();
                        list.push(n);
                        continue 'iterate_chars;
                    }
                }
            }
            list
        })
        .map(|numbers| numbers[0] * 10 + numbers[numbers.len() - 1])
        .sum::<u32>();
    result
}

fn main() {
    let input = include_str!("../../inputs/day01.input");
    let result = calculate_calibration_value(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = include_str!("../../inputs/day01b.test");
        let result = calculate_calibration_value(data);
        assert_eq!(result, 281);
    }
}
