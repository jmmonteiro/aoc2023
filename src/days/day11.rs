use crate::utils::structs::{Answer, Solver};
use std::time::Instant;

pub struct Day;
/// Manhattan distance
fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
    (a.0 as i32 - b.0 as i32).abs() as usize + (a.1 as i32 - b.1 as i32).abs() as usize
}

fn expand(
    galaxy_positions: Vec<(usize, usize)>,
    increase: usize,
    split_vec: &Vec<Vec<&str>>,
) -> Vec<(usize, usize)> {
    let mut output = galaxy_positions.clone();
    // Add empty rows to galaxy positions
    for (j, r) in split_vec.iter().enumerate().rev() {
        if r.iter().all(|x| *x == ".") {
            output = output
                .iter()
                .map(|(jj, ii)| {
                    if jj > &j {
                        return (*jj + increase, *ii);
                    }
                    (*jj, *ii)
                })
                .collect::<Vec<(usize, usize)>>();
        }
    }

    // Add empty columns to galaxy positions
    for i in (0..split_vec[0].len()).rev() {
        if (0..split_vec.len()).all(|j| split_vec[j][i] == ".") {
            output = output
                .iter()
                .map(|(jj, ii)| {
                    if ii > &i {
                        return (*jj, *ii + increase);
                    }
                    (*jj, *ii)
                })
                .collect::<Vec<(usize, usize)>>();
        }
    }
    output
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let split_vec: Vec<Vec<&str>> = vec
            .iter()
            .map(|x| x.split("").filter(|x| x.len() > 0).collect())
            .collect();

        let mut galaxy_positions: Vec<(usize, usize)> = vec![];
        for (j, r) in split_vec.iter().enumerate() {
            for (i, s) in r.iter().enumerate() {
                if *s == "#" {
                    galaxy_positions.push((j as usize, i as usize));
                }
            }
        }

        let galaxy_positions = expand(galaxy_positions, 1, &split_vec);

        let answer: usize = (0..galaxy_positions.len() - 1)
            .map(|i| {
                ((i + 1)..galaxy_positions.len())
                    .map(|j| dist(galaxy_positions[i], galaxy_positions[j]))
                    .sum::<usize>()
            })
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let split_vec: Vec<Vec<&str>> = vec
            .iter()
            .map(|x| x.split("").filter(|x| x.len() > 0).collect())
            .collect();

        let mut galaxy_positions: Vec<(usize, usize)> = vec![];
        for (j, r) in split_vec.iter().enumerate() {
            for (i, s) in r.iter().enumerate() {
                if *s == "#" {
                    galaxy_positions.push((j as usize, i as usize));
                }
            }
        }

        let galaxy_positions = expand(galaxy_positions, 999_999, &split_vec);

        let answer: usize = (0..galaxy_positions.len() - 1)
            .map(|i| {
                ((i + 1)..galaxy_positions.len())
                    .map(|j| dist(galaxy_positions[i], galaxy_positions[j]) as usize)
                    .sum::<usize>()
            })
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day11::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/11/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "374");
    }
}
