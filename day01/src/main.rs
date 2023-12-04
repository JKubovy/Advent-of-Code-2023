fn first_part(input: &str) -> u32 {
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

fn second_part(input: &str) -> u32 {
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
        let data = include_str!("../inputs/test_first.txt");
        let result = first_part(data);
        assert_eq!(result, 142);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 54968);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test_second.txt");
        let result = second_part(data);
        assert_eq!(result, 281);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 54094);
    }
}
