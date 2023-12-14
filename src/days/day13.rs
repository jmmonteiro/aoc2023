//! Point of Incidence
//!
//! To summarize your pattern notes, add up the number of columns to the left of each vertical
//! line of reflection; to that, also add 100 multiplied by the number of rows above each
//! horizontal line of reflection. In the above example, the first pattern's vertical line
//! has 5 columns to its left and the second pattern's horizontal line has 4 rows above it,
//! a total of 405.
use crate::utils::structs::{Answer, Solver};
use std::time::Instant;

pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut answer: usize = 0;
        let mut field: Vec<Vec<&str>> = vec![];

        for l in vec.iter() {
            let parse_line: Vec<&str> = l.split("").filter(|s| s.len() > 0).collect();
            if parse_line.len() > 0 {
                field.push(parse_line);
            } else if field.len() > 0 {
                let mut sucess = false;
                // Check rows first
                for j in 1..field.len() {
                    let size = j.min(field.len() - j);

                    let lower_range = ((j - size)..j).rev();
                    let upper_range = j..(j + size);

                    sucess = lower_range
                        .zip(upper_range)
                        .map(|(j_up, j_down)| {
                            field[j_up]
                                .clone()
                                .into_iter()
                                .zip(field[j_down].clone().into_iter())
                                .all(|(a, b)| a == b)
                        })
                        .all(|x| x);

                    if sucess {
                        answer += 100 * j;
                        field.clear();
                        break;
                    }
                }

                if sucess {
                    continue;
                }
                // Rows did not work, check cols
                for i in 1..field[0].len() {
                    let size = i.min(field[0].len() - i);

                    let lower_range = ((i - size)..i).rev();
                    let upper_range = i..(i + size);

                    sucess = lower_range
                        .zip(upper_range)
                        .map(|(i_left, i_right)| {
                            field
                                .clone()
                                .into_iter()
                                .map(|row| row[i_left])
                                .zip(field.clone().into_iter().map(|row| row[i_right]))
                                .all(|(a, b)| a == b)
                        })
                        .all(|x| x);

                    if sucess {
                        answer += i;
                        field.clear();
                        break;
                    }
                }
            }
        }

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut answer: usize = 0;
        let mut field: Vec<Vec<&str>> = vec![];

        for l in vec.iter() {
            let parse_line: Vec<&str> = l.split("").filter(|s| s.len() > 0).collect();
            if parse_line.len() > 0 {
                field.push(parse_line);
            } else if field.len() > 0 {
                let mut sucess = false;
                // Check rows first
                for j in 1..field.len() {
                    let size = j.min(field.len() - j);

                    let lower_range = ((j - size)..j).rev();
                    let upper_range = j..(j + size);

                    sucess = lower_range
                        .zip(upper_range)
                        .map(|(j_up, j_down)| {
                            field[j_up]
                                .clone()
                                .into_iter()
                                .zip(field[j_down].clone().into_iter())
                                .filter(|(a, b)| a != b)
                                .count()
                        })
                        .sum::<usize>()
                        == 1;

                    if sucess {
                        answer += 100 * j;
                        field.clear();
                        break;
                    }
                }

                if sucess {
                    continue;
                }
                // Rows did not work, check cols
                for i in 1..field[0].len() {
                    let size = i.min(field[0].len() - i);

                    let lower_range = ((i - size)..i).rev();
                    let upper_range = i..(i + size);

                    sucess = lower_range
                        .zip(upper_range)
                        .map(|(i_left, i_right)| {
                            field
                                .clone()
                                .into_iter()
                                .map(|row| row[i_left])
                                .zip(field.clone().into_iter().map(|row| row[i_right]))
                                .filter(|(a, b)| a != b)
                                .count()
                        })
                        .sum::<usize>()
                        == 1;

                    if sucess {
                        answer += i;
                        field.clear();
                        break;
                    }
                }
            }
        }

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day13::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/13/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "405");
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/13/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "400");
    }
}
