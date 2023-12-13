enum SpringRecord {
    Operational,
    Damaged,
    Unknown,
}

fn parse_line(line: &str) -> (Vec<SpringRecord>, Vec<usize>) {
    let (records, numbers) = line.split_once(' ').expect("Wrong line format");
    (
        records
            .chars()
            .map(|c| match c {
                '.' => SpringRecord::Operational,
                '#' => SpringRecord::Damaged,
                '?' => SpringRecord::Unknown,
                _ => panic!("Unknow record"),
            })
            .collect(),
        numbers
            .split(',')
            .map(|c| c.parse().expect("Can't parse number"))
            .collect(),
    )
}

fn count_possibilities(mut records: Vec<SpringRecord>, count_errors: &[usize]) -> usize {
    records.push(SpringRecord::Operational);
    let mut dp_array = vec![
        vec![vec![Some(0usize); records.len() + 2]; count_errors.len() + 2];
        records.len() + 1
    ];
    dp_array[0][0][0] = Some(1);
    for pos in 0..records.len() {
        for error_count in 0..count_errors.len() + 1 {
            for len in 0..records.len() + 1 {
                let current = dp_array[pos][error_count][len];
                if current.is_none() {
                    continue;
                }
                if matches!(
                    records[pos],
                    SpringRecord::Operational | SpringRecord::Unknown
                ) {
                    if len == 0 || (error_count > 0 && len == count_errors[error_count - 1]) {
                        dp_array[pos + 1][error_count][0] =
                            Some(dp_array[pos + 1][error_count][0].unwrap_or(0) + current.unwrap());
                    }
                }
                if matches!(records[pos], SpringRecord::Damaged | SpringRecord::Unknown) {
                    let x = if len == 0 { 1 } else { 0 };
                    dp_array[pos + 1][error_count + x][len + 1] = Some(
                        dp_array[pos + 1][error_count + x][len + 1].unwrap_or(0) + current.unwrap(),
                    );
                }
            }
        }
    }
    dp_array[records.len()][count_errors.len()][0].unwrap_or(0)
}

fn expand_line(line: &str) -> String {
    let (records, numbers) = line.split_once(' ').expect("Wrong line format");
    let records = vec![records; 5].join("?");
    let numbers = vec![numbers; 5].join(",");
    vec![records, numbers].join(" ")
}

fn first_part(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(|(records, count_errors)| count_possibilities(records, &count_errors))
        .sum()
}

fn second_part(input: &str) -> usize {
    input
        .lines()
        .map(|line| expand_line(line))
        .map(|line| parse_line(&line))
        .map(|(records, count_errors)| count_possibilities(records, &count_errors))
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
        assert_eq!(result, 21);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 7674);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 525152);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 4443895258186);
    }

    #[test]
    fn test_expand_line() {
        let line = ".# 1";
        assert_eq!(expand_line(line).as_str(), ".#?.#?.#?.#?.# 1,1,1,1,1");
    }
}
