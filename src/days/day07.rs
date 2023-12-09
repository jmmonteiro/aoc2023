use crate::utils::structs::{Answer, Solver};
use itertools::Itertools;
use std::{cmp::Ordering, time::Instant};
pub struct Day;

struct Hand {
    cards: Vec<u8>,
    bet: u128,
    jokers: bool,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Check card frequency
        let mut self_card_freq = self.cards.iter().counts();
        let mut other_card_freq = other.cards.iter().counts();

        // Make jokers equal to the most frequent card.
        if self.jokers {
            if self_card_freq.contains_key(&1) {
                let self_most_frequent_card = self_card_freq.iter().fold((0, 0), |acc, (c, f)| {
                    if (*f > acc.1) && (**c != 1) {
                        return (**c, *f);
                    }
                    acc
                });
                let n = *self_card_freq.get(&1).unwrap();
                match self_card_freq.get_mut(&self_most_frequent_card.0) {
                    Some(c) => {
                        *c += n;
                        self_card_freq.remove(&1);
                    }
                    None => {}
                }
            }
            if other_card_freq.contains_key(&1) {
                let other_most_frequent_card =
                    other_card_freq.iter().fold((0, 0), |acc, (c, f)| {
                        if (*f > acc.1) && (**c != 1) {
                            return (**c, *f);
                        }
                        acc
                    });
                let n = *other_card_freq.get(&1).unwrap();
                match other_card_freq.get_mut(&other_most_frequent_card.0) {
                    Some(c) => {
                        *c += n;
                        other_card_freq.remove(&1);
                    }
                    None => {}
                }
            }
        }

        // If one hand has less unique values, it automatically makes it higher
        if self_card_freq.len() < other_card_freq.len() {
            return Ordering::Greater;
        }
        if self_card_freq.len() > other_card_freq.len() {
            return Ordering::Less;
        }

        // Check the largest group size
        let self_card_freq = self_card_freq.into_values().max().unwrap();
        let other_card_freq = other_card_freq.into_values().max().unwrap();
        if self_card_freq > other_card_freq {
            return Ordering::Greater;
        }
        if self_card_freq < other_card_freq {
            return Ordering::Less;
        }

        // We have to check the highest first card
        for (s, o) in self.cards.iter().zip(other.cards.iter()) {
            if s > o {
                return Ordering::Greater;
            }
            if s < o {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        fn map_cards(card: &str) -> u8 {
            match card.parse::<u8>() {
                Ok(c) => c,
                Err(_) => match card {
                    "T" => 10,
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    "A" => 14,
                    _ => {
                        panic!("Card {} is not recognised.", card);
                    }
                },
            }
        }

        let answer: u128 = vec
            .iter()
            .map(|h| {
                let hand_score: Vec<&str> = h.split(" ").collect();
                Hand {
                    cards: hand_score[0]
                        .chars()
                        .map(|s| map_cards(&s.to_string()))
                        .collect::<Vec<u8>>(),
                    bet: hand_score[1].parse::<u128>().unwrap(),
                    jokers: false,
                }
            })
            .sorted()
            .zip(1..(vec.len() + 1))
            .map(|(hand, rank)| hand.bet * (rank as u128))
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        fn map_cards(card: &str) -> u8 {
            match card.parse::<u8>() {
                Ok(c) => c,
                Err(_) => match card {
                    "J" => 1,
                    "T" => 10,
                    "Q" => 11,
                    "K" => 12,
                    "A" => 13,
                    _ => {
                        panic!("Card {} is not recognised.", card);
                    }
                },
            }
        }

        let answer: u128 = vec
            .iter()
            .map(|h| {
                let hand_score: Vec<&str> = h.split(" ").collect();
                Hand {
                    cards: hand_score[0]
                        .chars()
                        .map(|s| map_cards(&s.to_string()))
                        .collect::<Vec<u8>>(),
                    bet: hand_score[1].parse::<u128>().unwrap(),
                    jokers: true,
                }
            })
            .sorted()
            .zip(1..(vec.len() + 1))
            .map(|(hand, rank)| hand.bet * (rank as u128))
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/07/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "6440")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/07/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "5905")
    }
}
