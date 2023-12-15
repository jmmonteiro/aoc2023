//! Haunted Wasteland
use crate::utils::structs::{Answer, Solver};
use core::panic;
use itertools::Itertools;
use std::{collections::HashMap, time::Instant};
pub struct Day;
use itertools::FoldWhile::{Continue, Done};

fn parse_nodes(vec: &Vec<String>) -> HashMap<&str, (&str, &str)> {
    vec.iter()
        .skip(2)
        .map(|s| {
            let info: Vec<&str> = s.split(" ").collect();
            (info[0], (&info[2][1..4], &info[3][..3]))
        })
        .collect()
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let directions: Vec<&str> = vec[0].split("").filter(|s| s.len() > 0).collect();
        let nodes = parse_nodes(&vec);

        let answer = directions
            .iter()
            .cycle()
            .enumerate()
            .fold_while((0 as usize, "AAA"), |acc, (i, d)| {
                let next_node = match *d {
                    "L" => nodes[acc.1].0,
                    "R" => nodes[acc.1].1,
                    _ => {
                        panic!("{}", format!("Unrecognized direction {}", d));
                    }
                };

                let output = (i, next_node);

                if next_node == "ZZZ" {
                    return Done(output);
                }
                Continue(output)
            })
            .into_inner()
            .0
            + 1;

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let directions: Vec<&str> = vec[0].split("").filter(|s| s.len() > 0).collect();
        let nodes = parse_nodes(&vec);

        let mut start_nodes: Vec<&str> = vec
            .iter()
            .skip(2)
            .map(|s| s.split(" ").next().unwrap())
            .filter(|x| x.ends_with("A"))
            .collect();

        let min_steps_to_end = start_nodes
            .iter()
            .map(|n| {
                directions
                    .iter()
                    .cycle()
                    .enumerate()
                    .fold_while((0 as usize, n), |acc, (i, d)| {
                        let next_node = match *d {
                            "L" => &nodes[acc.1].0,
                            "R" => &nodes[acc.1].1,
                            _ => {
                                panic!("{}", format!("Unrecognized direction {}", d));
                            }
                        };

                        let output = (i, next_node);

                        if next_node.ends_with("Z") {
                            return Done(output);
                        }
                        Continue(output)
                    })
                    .into_inner()
                    .0
                    + 1
            })
            .collect::<Vec<usize>>();

        // Get Least Common Multiple for all pairs of start nodes
        fn gcd(a: &mut usize, b: &mut usize) -> usize {
            let mut t = *b;
            while *b != 0 {
                t = *b;
                *b = *a % *b;
                *a = t;
            }
            *a
        }
        fn lcm(a: usize, b: usize) -> usize {
            let mut amut = a;
            let mut bmut = b;
            (a * b) / gcd(&mut amut, &mut bmut)
        }

        let answer = min_steps_to_end
            .iter()
            .fold(1, |a, b| lcm(a.clone(), b.clone()));

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/08/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "6")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/08/test_input_2.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "6")
    }
}
