const HIGH_CARD: u32 = 1;
const ONE_PAIR: u32 = 2;
const TWO_PAIR: u32 = 3;
const THREE_OF_A_KIND: u32 = 4;
const FULL_HOUSE: u32 = 5;
const FOUR_OF_A_KIND: u32 = 6;
const FIVE_OF_A_KIND: u32 = 7;

fn get_card_priority(card: &char) -> u32 {
    if let Some(n) = card.to_digit(10) {
        return n;
    }
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unknown card"),
    }
}

fn get_numeric_representation_of_cards(cards: &str, joker: bool) -> u64 {
    cards
        .chars()
        .fold(vec!['1'], |mut acc, c| {
            if c.is_ascii_digit() {
                acc.push('0');
                acc.push(c);
            } else if joker && c == 'J' {
                acc.push('0');
                acc.push('1');
            } else {
                acc.push('1');
                match c {
                    'T' => acc.push('0'),
                    'J' => acc.push('1'),
                    'Q' => acc.push('2'),
                    'K' => acc.push('3'),
                    'A' => acc.push('4'),
                    _ => panic!("Unknown card"),
                }
            }
            acc
        })
        .into_iter()
        .collect::<String>()
        .parse()
        .expect("Wrong hand cards")
}

fn get_combination_priority(mut counts: [u8; 15], joker: bool) -> u32 {
    let mut hand_priority = HIGH_CARD;
    let joker_index = get_card_priority(&'J') as usize;
    let jokers = counts[joker_index];
    if joker {
        counts[joker_index] = 0
    }
    for c in counts.into_iter() {
        match (c, hand_priority) {
            (2, ONE_PAIR) => hand_priority = TWO_PAIR,
            (2, THREE_OF_A_KIND) => hand_priority = FULL_HOUSE,
            (2, _) => hand_priority = ONE_PAIR,
            (3, ONE_PAIR) => hand_priority = FULL_HOUSE,
            (3, _) => hand_priority = THREE_OF_A_KIND,
            (4, _) => hand_priority = FOUR_OF_A_KIND,
            (5, _) => hand_priority = FIVE_OF_A_KIND,
            _ => {}
        }
    }
    if joker {
        match (jokers, hand_priority) {
            (4 | 5, _) => hand_priority = FIVE_OF_A_KIND,
            (3, ONE_PAIR) => hand_priority = FIVE_OF_A_KIND,
            (3, _) => hand_priority = FOUR_OF_A_KIND,
            (2, THREE_OF_A_KIND) => hand_priority = FIVE_OF_A_KIND,
            (2, ONE_PAIR) => hand_priority = FOUR_OF_A_KIND,
            (2, _) => hand_priority = THREE_OF_A_KIND,
            (1, FOUR_OF_A_KIND) => hand_priority = FIVE_OF_A_KIND,
            (1, THREE_OF_A_KIND) => hand_priority = FOUR_OF_A_KIND,
            (1, TWO_PAIR) => hand_priority = FULL_HOUSE,
            (1, ONE_PAIR) => hand_priority = THREE_OF_A_KIND,
            (1, _) => hand_priority = ONE_PAIR,
            _ => {}
        };
    }
    hand_priority
}

fn get_combination(cards: &str, joker: bool) -> (u32, u64) {
    let mut card_priority_count = [0u8; 15];
    cards
        .chars()
        .for_each(|card| card_priority_count[get_card_priority(&card) as usize] += 1);
    (
        get_combination_priority(card_priority_count, joker),
        get_numeric_representation_of_cards(cards, joker),
    )
}

fn solve(input: &str, joker: bool) -> u64 {
    let mut games = input
        .lines()
        .map(|line| {
            let mut words = line.split_ascii_whitespace();
            let (combination, priority) =
                get_combination(words.next().expect("Unable to parse cards"), joker);
            (
                combination,
                priority,
                words
                    .next()
                    .expect("No bet")
                    .parse::<u64>()
                    .expect("Bet is not a number"),
            )
        })
        .collect::<Vec<_>>();
    games.sort_unstable_by_key(|(combo, priority, _)| (*combo, *priority));
    games
        .iter()
        .fold((1, 0), |(index, sum), (_, _, bet)| {
            (index + 1, sum + index * bet)
        })
        .1
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let first_part = solve(input, false);
    println!("First part: {}", first_part);
    let second_part = solve(input, true);
    println!("Second part: {}", second_part);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let data = include_str!("../inputs/test.txt");
        let result = solve(data, false);
        assert_eq!(result, 6440);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = solve(data, false);
        assert_eq!(result, 251121738);
    }

    #[test]
    fn test_get_card_priority() {
        assert!(get_card_priority(&'1') < get_card_priority(&'2'));
        assert!(get_card_priority(&'8') < get_card_priority(&'9'));
        assert!(get_card_priority(&'9') < get_card_priority(&'T'));
        assert!(get_card_priority(&'T') < get_card_priority(&'J'));
        assert!(get_card_priority(&'J') < get_card_priority(&'Q'));
        assert!(get_card_priority(&'Q') < get_card_priority(&'K'));
        assert!(get_card_priority(&'K') < get_card_priority(&'A'));
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = solve(data, true);
        assert_eq!(result, 5905);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = solve(data, true);
        assert_eq!(result, 251421071);
    }
}
