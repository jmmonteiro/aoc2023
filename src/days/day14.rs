//! Parabolic Reflector Dish
use crate::utils::structs::{Answer, Solver};
use pathfinding::directed::cycle_detection;
use std::time::Instant;

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut map: Vec<Vec<&str>> = vec
            .into_iter()
            .map(|s| s.split("").into_iter().filter(|x| x.len() > 0).collect())
            .collect();

        for i in 0..vec[0].len() {
            for j in 0..vec.len() {
                if map[j][i] == "O" {
                    for k in (0..j).rev() {
                        if map[k][i] == "#" || map[k][i] == "O" {
                            break;
                        }
                        map[k][i] = "O";
                        map[k + 1][i] = ".";
                    }
                }
            }
        }

        let answer: usize = (1..map.len() + 1)
            .rev()
            .zip(map.into_iter())
            .map(|(row_number, row)| row_number * row.into_iter().filter(|x| *x == "O").count())
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut map: Vec<Vec<&str>> = vec
            .into_iter()
            .map(|s| s.split("").into_iter().filter(|x| x.len() > 0).collect())
            .collect();

        let first_value = (1..map.len() + 1)
            .rev()
            .zip(map.clone().into_iter())
            .map(|(row_number, row)| row_number * row.into_iter().filter(|x| *x == "O").count())
            .sum();

        fn cycle_dish(input: (usize, Vec<Vec<&str>>)) -> (usize, Vec<Vec<&str>>) {
            let mut map = input.1.clone();

            // Cycle north
            for i in 0..map[0].len() {
                for j in 0..map.len() {
                    if map[j][i] == "O" {
                        for k in (0..j).rev() {
                            if map[k][i] == "#" || map[k][i] == "O" {
                                break;
                            }
                            map[k][i] = "O";
                            map[k + 1][i] = ".";
                        }
                    }
                }
            }
            // Cycle west
            for j in 0..map.len() {
                for i in 0..map[0].len() {
                    if map[j][i] == "O" {
                        for k in (0..i).rev() {
                            if map[j][k] == "#" || map[j][k] == "O" {
                                break;
                            }
                            map[j][k] = "O";
                            map[j][k + 1] = ".";
                        }
                    }
                }
            }
            // Cycle south
            for i in 0..map[0].len() {
                for j in (0..map.len()).rev() {
                    if map[j][i] == "O" {
                        for k in (j + 1)..map.len() {
                            if map[k][i] == "#" || map[k][i] == "O" {
                                break;
                            }
                            map[k][i] = "O";
                            map[k - 1][i] = ".";
                        }
                    }
                }
            }

            // Cycle east
            for j in 0..map.len() {
                for i in (0..map[0].len()).rev() {
                    if map[j][i] == "O" {
                        for k in (i + 1)..map[0].len() {
                            if map[j][k] == "#" || map[j][k] == "O" {
                                break;
                            }
                            map[j][k] = "O";
                            map[j][k - 1] = ".";
                        }
                    }
                }
            }

            (
                (1..map.len() + 1)
                    .rev()
                    .zip(map.clone().into_iter())
                    .map(|(row_number, row)| {
                        row_number * row.into_iter().filter(|x| *x == "O").count()
                    })
                    .sum::<usize>(),
                map,
            )
        }

        let (size, _, first_element) =
            cycle_detection::floyd((first_value, map.clone()), cycle_dish);

        let total_cycles = (1_000_000_000 % size) + first_element + 2;

        let mut load = first_value;
        dbg!(load);
        for _ in 0..total_cycles {
            (load, map) = cycle_dish((load, map));
            dbg!(load);
        }

        dbg!(size, first_element, total_cycles);

        return Some(Answer::new(load.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day14::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/14/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "136");
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/14/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "64");
    }
}
