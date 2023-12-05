use crate::utils::structs::{Answer, Solver};
use regex::Regex;
use std::{collections::HashSet, time::Instant};
pub struct Day;
use itertools::Itertools;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let re_number = Regex::new(r"\d+").unwrap();

        let total: u32 = vec
            .into_iter()
            .map(|l| {
                let numbers: Vec<&str> =
                    re_number.find_iter(l).skip(1).map(|x| x.as_str()).collect();
                let diff: i32 =
                    ((numbers.len() as i32) - HashSet::<&str>::from_iter(numbers).len() as i32) - 1;
                if diff < 0 {
                    return 0;
                }
                (2 as u32).pow(diff as u32)
            })
            .sum();

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let re_number = Regex::new(r"\d+").unwrap();

        fn process_card(
            mut total: i32,
            cards: &Vec<String>,
            start_idx: usize,
            end_idx: usize,
            re_number: &Regex,
            initial_size: i32,
            card_results: &Vec<usize>,
        ) -> i32 {
            let mut i = start_idx;
            for _ in start_idx..end_idx {
                total += 1;

                if card_results[i] > 0 {
                    total = process_card(
                        total,
                        &cards,
                        (i + 1).min(cards.len() - 1),
                        (i + 1 + card_results[i]).min(cards.len()),
                        re_number,
                        initial_size,
                        &card_results,
                    );
                }
                i += 1;
            }
            total
        }

        // Get initial size
        let initial_size = re_number
            .find_iter(&vec[0])
            .skip(1)
            .map(|x| x.as_str())
            .count();

        // Pre-compute card results
        let card_results: Vec<usize> = vec
            .into_iter()
            .map(|l| {
                let diff: i32 = (initial_size as i32)
                    - re_number
                        .find_iter(l)
                        .skip(1)
                        .map(|x| x.as_str())
                        .unique()
                        .count() as i32;
                diff.max(0) as usize
            })
            .collect();

        let total = process_card(
            0,
            &vec,
            0,
            vec.len(),
            &re_number,
            initial_size as i32,
            &card_results,
        );

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day04::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/04/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "13")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/04/test_input_2.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "30")
    }
}
