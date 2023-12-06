fn first_part(input: &str) -> u64 {
    parse_races(input)
        .iter()
        .map(|(time, distance)| calculate_win_combinations(time, distance))
        .reduce(|acc, a| acc * a)
        .expect("No races given")
}

fn calculate_win_combinations(time: &u64, distance: &u64) -> u64 {
    let (lower, upper);
    {
        let (time, distance) = (*time as f64, *distance as f64);
        let x = f64::sqrt(time * time - 4_f64 * distance);
        (lower, upper) = (((time - x) / 2_f64).ceil(), ((time + x) / 2_f64).floor());
    }
    let (mut lower, mut upper) = (lower as u64, upper as u64);
    if lower * (*time - lower) == *distance {
        // if using lower bound match longest distance add one
        lower += 1;
    }
    if upper * (*time - upper) == *distance {
        // if using upper bound match longest distance sub one
        upper -= 1;
    }
    (upper + 1).saturating_sub(lower)
}

fn second_part(input: &str) -> u64 {
    let (time, distance) = parse_races_second(input);
    calculate_win_combinations(&time, &distance)
}

fn parse_races(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    if let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()) {
        return first_line
            .split_ascii_whitespace()
            .skip(1)
            .zip(second_line.split_ascii_whitespace().skip(1))
            .map(|(t, d)| {
                (
                    t.parse::<u64>().expect("Time too large"),
                    d.parse::<u64>().expect("Distance too large"),
                )
            })
            .collect();
    }
    panic!("Can't parse races")
}

fn parse_races_second(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    if let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()) {
        return (
            first_line
                .replace(' ', "")
                .split_once(':')
                .expect("Wrong format of first line")
                .1
                .parse()
                .expect("Time is too large"),
            second_line
                .replace(' ', "")
                .split_once(':')
                .expect("Wrong format of second line")
                .1
                .parse()
                .expect("Distance is too large"),
        );
    }
    panic!("Can't parse races")
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
        assert_eq!(result, 288);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 2756160);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 71503);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 34788142);
    }
}
