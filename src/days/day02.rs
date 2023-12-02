use crate::utils::structs::{Answer, Solver};
use regex::Regex;
use std::time::Instant;
pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut total: usize = 0;
        let re_red = Regex::new(r"(\d+) red").unwrap();
        let re_blue = Regex::new(r"(\d+) blue").unwrap();
        let re_green = Regex::new(r"(\d+) green").unwrap();

        let re_number = Regex::new(r"\d+").unwrap();

        for (i, l) in vec.into_iter().enumerate() {
            let n_red: Vec<usize> = re_red
                .find_iter(l)
                .map(|x| re_number.find_at(x.as_str(), 0))
                .map(|y| y.unwrap().as_str().parse::<usize>().unwrap())
                .collect();

            if n_red.iter().any(|n| *n > 12) {
                continue;
            }

            let n_green: Vec<usize> = re_green
                .find_iter(l)
                .map(|x| re_number.find_at(x.as_str(), 0))
                .map(|y| y.unwrap().as_str().parse::<usize>().unwrap())
                .collect();

            if n_green.iter().any(|n| *n > 13) {
                continue;
            }

            let n_blue: Vec<usize> = re_blue
                .find_iter(l)
                .map(|x| re_number.find_at(x.as_str(), 0))
                .map(|y| y.unwrap().as_str().parse::<usize>().unwrap())
                .collect();

            if n_blue.iter().any(|n| *n > 14) {
                continue;
            }

            total += i + 1;
        }

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let mut total: u128 = 0;
        let mut power: u128;
        let re_red = Regex::new(r"(\d+) red").unwrap();
        let re_blue = Regex::new(r"(\d+) blue").unwrap();
        let re_green = Regex::new(r"(\d+) green").unwrap();

        let re_number = Regex::new(r"\d+").unwrap();

        for l in vec {
            power = 1;
            power *= re_red
                .find_iter(l)
                .map(|x| re_number.find_at(x.as_str(), 0))
                .map(|y| y.unwrap().as_str().parse::<u128>().unwrap())
                .max()
                .unwrap();
            power *= re_green
                .find_iter(l)
                .map(|x| re_number.find_at(x.as_str(), 0))
                .map(|y| y.unwrap().as_str().parse::<u128>().unwrap())
                .max()
                .unwrap();
            power *= re_blue
                .find_iter(l)
                .map(|x| re_number.find_at(x.as_str(), 0))
                .map(|y| y.unwrap().as_str().parse::<u128>().unwrap())
                .max()
                .unwrap();

            total += power;
        }

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day02::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/02/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "8")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/02/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "2286")
    }
}
