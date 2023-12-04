struct Card {
    index: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

struct Win {
    index: usize,
    wins: usize,
}

fn get_numbers(text: &str) -> Vec<usize> {
    text.split_ascii_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect()
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let colon_index = line.find(':').unwrap();
            let index = line[..colon_index]
                .split_ascii_whitespace()
                .collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap();
            let (winning, rest) = line[colon_index + 2..].split_once('|').unwrap();
            let winning_numbers = get_numbers(winning);
            let numbers = get_numbers(rest);
            Card {
                index,
                winning_numbers,
                numbers,
            }
        })
        .collect()
}

fn first_part(cards: &[Card]) -> u32 {
    cards
        .iter()
        .filter_map(|card| {
            card.numbers.iter().fold(None, |acc, num| {
                if card.winning_numbers.contains(num) {
                    return acc.map(|i| i << 1).or(Some(1));
                }
                acc
            })
        })
        .sum()
}

fn second_part(cards: &[Card]) -> u32 {
    let wins = cards
        .iter()
        .map(|card| {
            let wins = card.numbers.iter().fold(0, |acc, num| {
                if card.winning_numbers.contains(num) {
                    return acc + 1;
                }
                acc
            });
            Win {
                index: card.index,
                wins,
            }
        })
        .collect::<Vec<_>>();
    let mut multiply = vec![1; wins.len()];
    for (index, win) in wins.iter().enumerate() {
        for i in index + 1..index + 1 + win.wins {
            multiply[i] += multiply[index];
        }
    }
    multiply.iter().sum()
}

fn main() {
    let input = include_str!("../../inputs/day04.input");
    let cards = parse_cards(input);
    let first_part = first_part(&cards);
    println!("First part: {}", first_part);
    let second_part = second_part(&cards);
    println!("Second part: {}", second_part);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let data = include_str!("../../inputs/day04.test");
        let cards = parse_cards(data);
        let result = first_part(&cards);
        assert_eq!(result, 13);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../../inputs/day04.input");
        let cards = parse_cards(data);
        let result = first_part(&cards);
        assert_eq!(result, 20829);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../../inputs/day04.test");
        let cards = parse_cards(data);
        let result = second_part(&cards);
        assert_eq!(result, 30);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../../inputs/day04.input");
        let cards = parse_cards(data);
        let result = second_part(&cards);
        assert_eq!(result, 12648035);
    }
}
