//! If You Give A Seed A Fertilizer
use crate::utils::structs::{Answer, Solver};
use std::{ops::Range, time::Instant};
pub struct Day;
use itertools::Itertools;
use rayon::prelude::*;

fn parse_maps(vec: &Vec<String>) -> Vec<Vec<Vec<u128>>> {
    let mut maps: Vec<Vec<Vec<u128>>> = vec![vec![]];
    let mut current_map: usize = 0;
    for i in 3..vec.len() {
        if vec[i].is_empty() {
            continue;
        }
        if vec[i].chars().nth(0).unwrap().is_alphabetic() {
            current_map += 1;
            maps.push(vec![]);
            continue;
        }

        maps[current_map].push(
            vec[i]
                .split_whitespace()
                .map(|x| x.parse::<u128>().expect("Failed to parse map"))
                .collect::<Vec<u128>>(),
        );
    }
    maps
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        // --- Parse seeds
        let mut seeds: Vec<u128> = vec[0]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u128>().expect("Failed to parse seeds."))
            .collect();

        // --- Parse maps
        let maps = parse_maps(&vec);

        // --- Map seeds
        let answer = seeds
            .iter_mut()
            .map(|s| {
                for m in &maps {
                    for row in m {
                        if (row[1]..(row[1] + row[2])).contains(s) {
                            *s = row[0] + (*s - row[1]);
                            break;
                        }
                    }
                }
                s
            })
            .min()
            .unwrap();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        // --- Parse seeds
        let mut seed_ranges: Vec<Range<u128>> = vec[0]
            .split_whitespace()
            .skip(1)
            .tuples()
            .map(|(a, b)| {
                let start = a.parse::<u128>().expect("Failed to parse seeds.");
                let range = b.parse::<u128>().expect("Failed to parse seeds.");
                start..(start + range)
            })
            .collect();

        // --- Parse maps
        let maps = parse_maps(&vec);

        // --- Map seeds
        let answer = seed_ranges
            .par_iter_mut()
            .map(|seeds| {
                seeds
                    .map(|mut s| {
                        for m in &maps {
                            for row in m {
                                if (row[1]..(row[1] + row[2])).contains(&s) {
                                    s = row[0] + (s - row[1]);
                                    break;
                                }
                            }
                        }
                        s
                    })
                    .fold(u128::MAX, |acc, e| {
                        if e < acc {
                            return e;
                        }
                        acc
                    })
            })
            .min()
            .unwrap();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day05::*;
    use crate::utils::input;
    #[test]
    fn part1_test_input() {
        let vec = input::read_file("inputs/05/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "35")
    }
    #[test]
    fn part2_test_input() {
        let vec = input::read_file("inputs/05/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "46")
    }
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/05/input.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "278755257")
    }
    // Too slow, improve algorithm first
    // #[test]
    // fn part2() {
    // let vec = input::read_file("inputs/05/input.txt");
    // assert_eq!(Day.part2(&vec).unwrap().answer, "26829166")
    // }
}
