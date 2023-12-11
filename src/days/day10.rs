use crate::utils::structs::{Answer, Solver};
use core::panic;
use std::time::Instant;

pub struct Day;

fn check_surrounding(
    j: usize,
    i: usize,
    visited: &Vec<Vec<bool>>,
    current_tile: &str,
    map_str: &Vec<Vec<&str>>,
) -> Option<Vec<(usize, usize)>> {
    let mut output = vec![];

    fn check_direction(
        visited: &Vec<Vec<bool>>,
        j: usize,
        i: usize,
        output: &mut Vec<(usize, usize)>,
        direction: &str,
        map_str: &Vec<Vec<&str>>,
    ) {
        match direction {
            "left" => {
                if i > 0 && !visited[j][i - 1] && map_str[j][i - 1] != "." {
                    output.push((j, i - 1));
                }
            }
            "right" => {
                if i < visited[0].len() - 1 && !visited[j][i + 1] && map_str[j][i + 1] != "." {
                    output.push((j, i + 1));
                }
            }
            "up" => {
                if j > 0 && !visited[j - 1][i] && map_str[j - 1][i] != "." {
                    output.push((j - 1, i));
                }
            }
            "down" => {
                if j < visited.len() - 1 && !visited[j + 1][i] && map_str[j + 1][i] != "." {
                    output.push((j + 1, i));
                }
            }
            _ => {
                panic!("Unrecognised direction: {}", direction);
            }
        }
    }

    match current_tile {
        "-" => {
            check_direction(&visited, j, i, &mut output, "left", &map_str);
            check_direction(&visited, j, i, &mut output, "right", &map_str);
        }
        "|" => {
            check_direction(&visited, j, i, &mut output, "up", &map_str);
            check_direction(&visited, j, i, &mut output, "down", &map_str);
        }
        "L" => {
            check_direction(&visited, j, i, &mut output, "up", &map_str);
            check_direction(&visited, j, i, &mut output, "right", &map_str);
        }
        "J" => {
            check_direction(&visited, j, i, &mut output, "up", &map_str);
            check_direction(&visited, j, i, &mut output, "left", &map_str);
        }
        "7" => {
            check_direction(&visited, j, i, &mut output, "down", &map_str);
            check_direction(&visited, j, i, &mut output, "left", &map_str);
        }
        "F" => {
            check_direction(&visited, j, i, &mut output, "down", &map_str);
            check_direction(&visited, j, i, &mut output, "right", &map_str);
        }
        "S" => {
            if visited.iter().any(|l| l.iter().any(|x| *x)) {
                // End of path
                return None;
            }
            // Start of path
            for d in ["up", "down", "left", "right"] {
                check_direction(&visited, j, i, &mut output, d, &map_str);
            }
        }
        _ => {
            panic!("Unrecognised tile: {}", current_tile);
        }
    };

    if output.len() == 0 {
        panic!("Could not find a direction to move to. This should never happen.")
    }
    Some(output)
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let map_str: Vec<Vec<&str>> = vec
            .iter()
            .map(|x| x.split("").filter(|x| x.len() > 0).collect())
            .collect();

        // Find start
        let mut start_tile: (usize, usize) = (0, 0);
        for i in 0..map_str[0].len() {
            for j in 0..map_str.len() {
                if map_str[j][i] == "S" {
                    start_tile = (j, i);
                    break;
                }
            }
        }

        // Create visited map
        let mut visited: Vec<Vec<bool>> = map_str
            .iter()
            .map(|x| x.iter().map(|_| false).collect())
            .collect();

        // Check paths
        let mut current_point = start_tile;
        let mut distance: usize = 0;
        loop {
            match check_surrounding(
                current_point.0,
                current_point.1,
                &visited,
                map_str[current_point.0][current_point.1],
                &map_str,
            ) {
                Some(s) => {
                    current_point = s[0];
                    visited[current_point.0][current_point.1] = true;
                    distance += 1;

                    // Hack to prevent it from going back on the first iteration
                    if distance == 1 {
                        visited[start_tile.0][start_tile.1] = true;
                    } else {
                        visited[start_tile.0][start_tile.1] = false;
                    }
                }
                None => break,
            };
        }

        return Some(Answer::new((distance / 2).to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day10::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/10/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "8");
    }
    // #[test]
    // fn part2() {
    // let vec = input::read_file("inputs/10/test_input_1.txt");
    // assert_eq!(Day.part2(&vec).unwrap().answer, "2")
    // }
}
