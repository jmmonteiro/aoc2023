use crate::utils::structs::{Answer, Solver};
use itertools::Itertools;
use std::time::Instant;
pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let answer: i64 = vec
            .iter()
            .map(|l| {
                let mut l_split: Vec<i64> = l
                    .split_whitespace()
                    .map(|c| c.parse::<i64>().expect("Could not parse line."))
                    .collect();
                let last_value = *l_split.last().unwrap();

                let mut last_iter: Vec<i64> = vec![];
                loop {
                    l_split = l_split.iter().tuple_windows().map(|(a, b)| b - a).collect();
                    last_iter.push(*l_split.last().expect("No items on l_split"));

                    if l_split.iter().all(|x| *x == 0) {
                        break;
                    }
                }
                last_iter.iter().rev().skip(1).fold(0, |acc, e| acc + e) + last_value
            })
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let answer: i64 = vec
            .iter()
            .map(|l| {
                let mut l_split: Vec<i64> = l
                    .split_whitespace()
                    .map(|c| c.parse::<i64>().expect("Could not parse line."))
                    .collect();
                let first_value = *l_split.first().unwrap();

                let mut first_iter: Vec<i64> = vec![];
                loop {
                    l_split = l_split.iter().tuple_windows().map(|(a, b)| b - a).collect();
                    first_iter.push(*l_split.first().expect("No items on l_split"));

                    if l_split.iter().all(|x| *x == 0) {
                        break;
                    }
                }
                first_value - first_iter.iter().rev().skip(1).fold(0, |acc, e| e - acc)
            })
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day09::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/09/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "114")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/09/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "2")
    }
}
