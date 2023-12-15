//! Gear Ratios
use crate::utils::structs::{Answer, Solver};
use regex::Regex;
use std::time::Instant;
pub struct Day;

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let mut total: u128 = 0;

        let re_number = Regex::new(r"\d+").unwrap();
        let re_symbol = Regex::new(r"[^0-9.]").unwrap();

        // First line and last line in separate (for extra performance)
        let mut numbers = re_number.find_iter(vec[0].as_str());
        for n in numbers {
            let start = ((n.start() as i32) - 1).max(0) as usize;
            let end = (n.end() + 1).min(vec[0].len());
            if re_symbol.is_match(vec[0].get(start..end).unwrap()) {
                total += n.as_str().parse::<u128>().unwrap();
                continue;
            }
            if re_symbol.is_match(vec[1].get(start..end).unwrap()) {
                total += n.as_str().parse::<u128>().unwrap();
                continue;
            }
        }

        for i_line in 1..vec.len() - 1 {
            numbers = re_number.find_iter(vec[i_line].as_str());
            for n in numbers {
                let start = ((n.start() as i32) - 1).max(0) as usize;
                let end = (n.end() + 1).min(vec[i_line].len());
                if re_symbol.is_match(vec[i_line].get(start..end).unwrap()) {
                    total += n.as_str().parse::<u128>().unwrap();
                    continue;
                }
                if re_symbol.is_match(vec[i_line - 1].get(start..end).unwrap()) {
                    total += n.as_str().parse::<u128>().unwrap();
                    continue;
                }
                if re_symbol.is_match(vec[i_line + 1].get(start..end).unwrap()) {
                    total += n.as_str().parse::<u128>().unwrap();
                    continue;
                }
            }
        }

        // First line and last line in separate (for extra performance)
        let i_line = vec.len() - 1;
        numbers = re_number.find_iter(vec[i_line].as_str());
        for n in numbers {
            let start = ((n.start() as i32) - 1).max(0) as usize;
            let end = (n.end() + 1).min(vec[i_line].len());
            if re_symbol.is_match(vec[i_line].get(start..end).unwrap()) {
                total += n.as_str().parse::<u128>().unwrap();
                continue;
            }
            if re_symbol.is_match(vec[i_line - 1].get(start..end).unwrap()) {
                total += n.as_str().parse::<u128>().unwrap();
                continue;
            }
        }

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();
        let mut total: u128 = 0;

        let re_number = Regex::new(r"\d+").unwrap();
        let re_gear = Regex::new(r"\*").unwrap();

        // First line and last line in separate (for extra performance)
        let mut gears = re_gear.find_iter(&vec[0]);
        let mut numbers = vec![];
        for g in gears {
            for n in re_number.find_iter(&vec[0]) {
                if n.end() == g.start() {
                    numbers.push(n.as_str().parse::<u128>().unwrap());
                } else if n.start() == g.end() {
                    numbers.push(n.as_str().parse::<u128>().unwrap());
                }
            }
            for n in re_number.find_iter(&vec[1]) {
                if n.end() >= g.start() && n.start() <= g.end() {
                    numbers.push(n.as_str().parse::<u128>().unwrap());
                }
            }
            if numbers.len() > 1 {
                total += numbers.iter().product::<u128>();
            }
            numbers.clear();
        }

        for i_line in 1..vec.len() - 1 {
            gears = re_gear.find_iter(&vec[i_line]);
            for g in gears {
                for n in re_number.find_iter(&vec[i_line]) {
                    if n.end() == g.start() {
                        numbers.push(n.as_str().parse::<u128>().unwrap());
                    } else if n.start() == g.end() {
                        numbers.push(n.as_str().parse::<u128>().unwrap());
                    }
                }
                for n in re_number.find_iter(&vec[i_line - 1]) {
                    if n.end() >= g.start() && n.start() <= g.end() {
                        numbers.push(n.as_str().parse::<u128>().unwrap());
                    }
                }
                for n in re_number.find_iter(&vec[i_line + 1]) {
                    if n.end() >= g.start() && n.start() <= g.end() {
                        numbers.push(n.as_str().parse::<u128>().unwrap());
                    }
                }
                if numbers.len() > 1 {
                    total += numbers.iter().product::<u128>();
                }
                numbers.clear();
            }
        }

        // First line and last line in separate (for extra performance)
        let i_line = vec.len() - 1;
        gears = re_gear.find_iter(&vec[i_line]);
        for g in gears {
            for n in re_number.find_iter(&vec[i_line]) {
                if n.end() == g.start() {
                    numbers.push(n.as_str().parse::<u128>().unwrap());
                } else if n.start() == g.end() {
                    numbers.push(n.as_str().parse::<u128>().unwrap());
                }
            }
            for n in re_number.find_iter(&vec[i_line - 1]) {
                if n.end() >= g.start() && n.start() <= g.end() {
                    numbers.push(n.as_str().parse::<u128>().unwrap());
                }
            }
            if numbers.len() > 1 {
                total += numbers.iter().product::<u128>();
            }
            numbers.clear();
        }

        return Some(Answer::new(total.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day03::*;
    use crate::utils::input;
    #[test]
    fn part1_test_input() {
        let vec = input::read_file("inputs/03/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "4361")
    }
    #[test]
    fn part2_test_input() {
        let vec = input::read_file("inputs/03/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "467835")
    }
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/03/input.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "526404")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/03/input.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "84399773")
    }
}
