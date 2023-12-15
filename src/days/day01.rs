//! Trebuchet?!
use crate::utils::structs::{Answer, Solver};
use regex::Regex;
use std::time::Instant;
pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut total: u32 = 0;
        let re = Regex::new(r"(\d)").unwrap();

        for l in vec {
            let digits: Vec<&str> = re.find_iter(l).map(|m| m.as_str()).collect();
            if digits.len() == 1 {
                total += format!("{}{}", &digits[0], &digits[0])
                    .parse::<u32>()
                    .unwrap();
            } else if digits.len() > 1 {
                total += format!("{}{}", &digits[0], &digits[digits.len() - 1])
                    .parse::<u32>()
                    .unwrap();
            }
        }

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut total: u32 = 0;
        let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

        fn convert_digit(digit: &str) -> &str {
            match digit {
                "one" => return "1",
                "two" => return "2",
                "three" => return "3",
                "four" => return "4",
                "five" => return "5",
                "six" => return "6",
                "seven" => return "7",
                "eight" => return "8",
                "nine" => return "9",
                _ => return digit,
            };
        }

        for l in vec {
            let digits: Vec<&str> = re.find_iter(l).map(|m| m.as_str()).collect();
            if digits.len() == 1 {
                let d1 = convert_digit(digits[0]);
                total += format!("{}{}", d1, d1).parse::<u32>().unwrap();
            } else if digits.len() > 1 {
                total += format!(
                    "{}{}",
                    convert_digit(digits[0]),
                    convert_digit(digits[digits.len() - 1])
                )
                .parse::<u32>()
                .unwrap();
            }
        }

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day01::*;
    use crate::utils::input;
    #[test]
    fn part1_test_input() {
        let vec = input::read_file("inputs/01/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "142")
    }
    #[test]
    fn part2_test_input() {
        let vec = input::read_file("inputs/01/test_input_2.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "281")
    }

    #[test]
    fn part1() {
        let vec = input::read_file("inputs/01/input.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "54561")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/01/input.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "54076")
    }
}
