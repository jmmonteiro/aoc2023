//! The Floor Will Be Lava

use crate::utils::structs::{Answer, Solver};
use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
struct Status {
    current_tile: (i8, i8),
    current_direction: (i8, i8),
    map: Vec<Vec<char>>,
    hit_map: Vec<Vec<HashSet<(i8, i8)>>>,
}

fn step(mut status: Status) -> Status {
    if status.hit_map[status.current_tile.0 as usize][status.current_tile.1 as usize]
        .contains(&status.current_direction)
    {
        return status;
    }

    status.hit_map[status.current_tile.0 as usize][status.current_tile.1 as usize]
        .insert(status.current_direction);

    if (status.current_tile.0 + status.current_direction.0) < 0
        || (status.current_tile.1 + status.current_direction.1) < 0
        || (status.current_tile.0 + status.current_direction.0
            >= (status.hit_map.len() as i8).try_into().unwrap())
        || (status.current_tile.1 + status.current_direction.1
            >= (status.hit_map[0].len() as i8).try_into().unwrap())
    {
        return status;
    }
    status.current_tile.0 += status.current_direction.0;
    status.current_tile.1 += status.current_direction.1;

    match status.map[status.current_tile.0 as usize][status.current_tile.1 as usize] {
        '\\' => {
            match status.current_direction {
                (-1, 0) => {
                    status.current_direction = (0, -1);
                }
                (1, 0) => {
                    status.current_direction = (0, 1);
                }
                (0, 1) => {
                    status.current_direction = (1, 0);
                }
                (0, -1) => {
                    status.current_direction = (-1, 0);
                }
                _ => {
                    panic!("Unrecognised direction: {:?}", status.current_direction);
                }
            }
            status = step(status);
        }
        '/' => {
            match status.current_direction {
                (-1, 0) => {
                    status.current_direction = (0, 1);
                }
                (1, 0) => {
                    status.current_direction = (0, -1);
                }
                (0, 1) => {
                    status.current_direction = (-1, 0);
                }
                (0, -1) => {
                    status.current_direction = (1, 0);
                }
                _ => {
                    panic!("Unrecognised direction: {:?}", status.current_direction);
                }
            }
            status = step(status);
        }
        '.' => {
            status = step(status);
        }
        '|' => match status.current_direction.1 {
            0 => {
                status = step(status);
            }
            1 | -1 => {
                let current_tile = status.current_tile;
                status.current_direction = (-1, 0);
                status = step(status);
                status.current_tile = current_tile;
                status.current_direction = (1, 0);
                status = step(status);
            }
            _ => {
                panic!("Unrecognised direction: {:?}", status.current_direction);
            }
        },
        '-' => match status.current_direction.0 {
            0 => {
                status = step(status);
            }
            1 | -1 => {
                let current_tile = status.current_tile;
                status.current_direction = (0, -1);
                status = step(status);
                status.current_tile = current_tile;
                status.current_direction = (0, 1);
                status = step(status);
            }
            _ => {
                panic!("Unrecognised direction: {:?}", status.current_direction);
            }
        },
        _ => {
            panic!(
                "Unrecognised tile: {:?}",
                status.map[status.current_tile.0 as usize][status.current_tile.1 as usize]
            );
        }
    }

    status
}

pub struct Day;
impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let map: Vec<Vec<char>> = vec
            .iter()
            .filter(|x| x.len() > 0)
            .map(|x| x.chars().collect())
            .collect();

        let hit_map = map
            .clone()
            .iter()
            .map(|x| x.iter().map(|_| HashSet::new()).collect())
            .collect::<Vec<Vec<HashSet<(i8, i8)>>>>();

        let mut status = Status {
            current_tile: (0, 0),
            current_direction: (0, 1),
            map,
            hit_map,
        };

        status = step(status);

        let answer: usize = status
            .hit_map
            .iter()
            .map(|x| x.iter().filter(|c| !c.is_empty()).count())
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let map: Vec<Vec<char>> = vec
            .iter()
            .filter(|x| x.len() > 0)
            .map(|x| x.chars().collect())
            .collect();

        let hit_map = map
            .clone()
            .iter()
            .map(|x| x.iter().map(|_| HashSet::new()).collect())
            .collect::<Vec<Vec<HashSet<(i8, i8)>>>>();

        let mut answer = 0;

        for j in 0..map.len() {
            let mut status = Status {
                current_tile: (j as i8, 0),
                current_direction: (0, 1),
                map: map.clone(),
                hit_map: hit_map.clone(),
            };

            status = step(status);

            let tmp: usize = status
                .hit_map
                .iter()
                .map(|x| x.iter().filter(|c| !c.is_empty()).count())
                .sum();

            if tmp > answer {
                answer = tmp;
            }
        }

        for j in 0..map.len() {
            let mut status = Status {
                current_tile: (j as i8, map[0].len() as i8 - 1),
                current_direction: (0, -1),
                map: map.clone(),
                hit_map: hit_map.clone(),
            };

            status = step(status);

            let tmp: usize = status
                .hit_map
                .iter()
                .map(|x| x.iter().filter(|c| !c.is_empty()).count())
                .sum();

            if tmp > answer {
                answer = tmp;
            }
        }

        for i in 0..map[0].len() {
            let mut status = Status {
                current_tile: (0, i as i8),
                current_direction: (1, 0),
                map: map.clone(),
                hit_map: hit_map.clone(),
            };

            status = step(status);

            let tmp: usize = status
                .hit_map
                .iter()
                .map(|x| x.iter().filter(|c| !c.is_empty()).count())
                .sum();

            if tmp > answer {
                answer = tmp;
            }
        }

        for i in 0..map[0].len() {
            let mut status = Status {
                current_tile: (map.len() as i8 - 1, i as i8),
                current_direction: (-1, 0),
                map: map.clone(),
                hit_map: hit_map.clone(),
            };

            status = step(status);

            let tmp: usize = status
                .hit_map
                .iter()
                .map(|x| x.iter().filter(|c| !c.is_empty()).count())
                .sum();

            if tmp > answer {
                answer = tmp;
            }
        }

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day16::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/16/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "46");
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/16/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "51");
    }
}
