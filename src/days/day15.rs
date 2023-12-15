//!  Lens Library
use itertools::Itertools;

use crate::utils::structs::{Answer, Solver};
use std::time::Instant;

pub struct Day;

fn hash_function(current_value: usize, new_character: char) -> usize {
    let mut output = current_value;

    // Convert to ASCII and increment
    output += new_character as usize;
    // Set the current value to itself multiplied by 17
    output *= 17;
    // Set the current value to the remainder of dividing itself by 256
    output %= 256;

    output
}

#[derive(Debug)]
struct Lens {
    box_num: usize,
    lens_code: String,
    operation: char,
    focal_len: Option<usize>,
}

fn get_details(input_str: &str) -> Lens {
    let lens_code = input_str.chars().filter(|c| c.is_alphabetic()).join("");

    let box_num = lens_code.chars().fold(0, |acc, e| hash_function(acc, e));

    let operation = *input_str
        .chars()
        .filter(|c| !c.is_alphanumeric())
        .collect::<Vec<char>>()
        .first()
        .expect("No operation found");

    let focal_len = input_str
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).expect("Unrecognised digit") as usize)
        .collect::<Vec<usize>>()
        .first()
        .map(|o| *o);

    Lens {
        box_num,
        lens_code,
        operation,
        focal_len,
    }
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let answer: usize = vec[0]
            .split(",")
            .filter(|x| x.len() > 0)
            .map(|step| step.chars().fold(0, |acc, e| hash_function(acc, e)))
            .sum();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| vec![]).collect();

        for input in vec[0].split(",") {
            let lens = get_details(input);

            // Check if lens is not in the box
            if (boxes[lens.box_num].len() == 0)
                || (boxes[lens.box_num]
                    .iter()
                    .filter(|l| *l.lens_code == lens.lens_code)
                    .count()
                    == 0)
            {
                if lens.operation == '=' {
                    boxes[lens.box_num].push(lens);
                }
                continue;
            };

            // Lens is in the box, check which operation we're dealing with
            match lens.operation {
                '-' => {
                    let index = boxes[lens.box_num]
                        .iter()
                        .position(|l| l.lens_code == lens.lens_code)
                        .expect("Lens not found, but it should be there");
                    boxes[lens.box_num].remove(index);
                }
                '=' => {
                    let index = boxes[lens.box_num]
                        .iter()
                        .position(|l| l.lens_code == lens.lens_code)
                        .expect("Lens not found, but it should be there");

                    let num = lens.box_num;
                    boxes[num][index] = lens;
                }
                _ => {
                    panic!("{} is not a recognised operation.", lens.operation);
                }
            }
        }

        let answer = (1..(boxes.len() + 1))
            .zip(boxes)
            .map(|(bn, b)| {
                b.iter()
                    .enumerate()
                    .map(|(i, l)| (i + 1) * l.focal_len.unwrap() * bn)
                    .sum::<usize>()
            })
            .sum::<usize>();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day15::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/15/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "1320");
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/15/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "145");
    }
}
