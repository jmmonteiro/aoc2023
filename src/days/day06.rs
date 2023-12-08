use crate::utils::structs::{Answer, Solver};
use std::time::Instant;
pub struct Day;
fn get_win_interval(t: f64, d: f64) -> u128 {
    let sqrt = (t.powf(2.0) - 4.0 * (-1.0) * (-d)).sqrt();
    let mut lower = (-t + sqrt) / (-2.0);
    let mut upper = (-t - sqrt) / (-2.0);

    // To account for the edge case of being exactly the value of the record
    if lower == lower.floor() {
        lower += 0.0001;
    }
    if upper == upper.ceil() {
        upper -= 0.0001;
    }

    (upper.floor() - lower.ceil()) as u128 + 1
}

impl Solver for Day {
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let times = vec[0]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<f64>().unwrap());
        let distances = vec[1]
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<f64>().unwrap());

        let answer: u128 = times
            .zip(distances)
            .map(|(t, d)| get_win_interval(t, d))
            .product();

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }

    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        let time = Instant::now();

        let answer: u128 = vec[0]
            .replace(" ", "")
            .split(":")
            .skip(1)
            .zip(vec[1].replace(" ", "").split(":").skip(1))
            .map(|(t, d)| get_win_interval(t.parse::<f64>().unwrap(), d.parse::<f64>().unwrap()))
            .collect::<Vec<u128>>()[0];

        return Some(Answer::new(answer.to_string(), time.elapsed()));
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day06::*;
    use crate::utils::input;
    #[test]
    fn part1() {
        let vec = input::read_file("inputs/06/test_input_1.txt");
        assert_eq!(Day.part1(&vec).unwrap().answer, "288")
    }
    #[test]
    fn part2() {
        let vec = input::read_file("inputs/06/test_input_1.txt");
        assert_eq!(Day.part2(&vec).unwrap().answer, "71503")
    }
}
