fn first_part(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .rev()
                .fold((0, Vec::new()), |(sum, diffs), number| {
                    if let Some(0) = diffs.last() {
                        return (sum, diffs);
                    }
                    let mut number = number.parse::<i64>().expect("Not a i64 number");
                    let mut new_diffs = Vec::new();
                    for diff in &diffs {
                        new_diffs.push(number);
                        number = diff - number;
                    }
                    new_diffs.push(number);
                    (sum + number, new_diffs)
                })
        })
        .map(|(sum, _)| sum)
        .sum()
}

fn second_part(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .fold((0, 1, Vec::new()), |(sum, sign, diffs), number| {
                    let mut number = number.parse::<i64>().expect("Not a i64 number");
                    let mut new_diffs = Vec::new();
                    for diff in &diffs {
                        new_diffs.push(number);
                        number -= diff;
                    }
                    new_diffs.push(number);
                    (sum + (sign * number), -sign, new_diffs)
                })
        })
        .map(|(sum, _, _)| sum)
        .sum()
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
        assert_eq!(result, 114);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 1834108701);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 2);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 993);
    }
}
