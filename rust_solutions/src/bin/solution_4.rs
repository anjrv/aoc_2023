use std::collections::HashSet;

fn main() {
    let result: u32 = std::fs::read_to_string("input4.txt")
        .unwrap()
        .split('\n')
        .map(|line| {
            let winning_cards = line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .split('|')
                .map(|numbers| {
                    numbers
                        .split_ascii_whitespace()
                        .map(|num| num.trim().parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .reduce(|winning, cards| winning.intersection(&cards).copied().collect())
                .unwrap();

            if winning_cards.is_empty() {
                0
            } else {
                u32::pow(2, winning_cards.len() as u32 - 1)
            }
        })
        .sum();

    println!("{:?}", result);
}
