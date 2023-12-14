use crate::utils::input;
use regex::Regex;
use std::time::Duration;
pub struct Answer {
    pub answer: String,
    pub time: Duration,
}

impl Answer {
    pub fn new(answer: String, time: Duration) -> Answer {
        Answer { answer, time }
    }
    pub fn display(&self) {
        if self.time.as_secs() > 0 {
            println!(
                "{}\t(runtime: {} s)",
                &self.answer,
                self.time.as_secs().to_string(),
            );
        } else if self.time.as_millis() > 0 {
            println!(
                "{}\t(runtime: {} ms)",
                &self.answer,
                self.time.as_millis().to_string(),
            );
        } else if self.time.as_micros() > 0 {
            println!(
                "{}\t(runtime: {} μs)",
                &self.answer,
                self.time.as_micros().to_string(),
            );
        } else {
            println!(
                "{}\t(runtime: {} ns)",
                &self.answer,
                self.time.as_nanos().to_string(),
            );
        }
    }
}

pub trait Solver {
    fn solve(&self, filepath: &str) -> Duration {
        let re = Regex::new(r"inputs/(\d+)/input.txt").unwrap();
        let cap = re.captures(filepath).expect("File not found.");
        assert!(cap.len() == 2);
        println!("------------- Day {} -------------", &cap[1]);

        let vec = input::read_file(filepath);
        let mut total_time = Duration::new(0, 0);
        match self.part1(&vec) {
            Some(i) => {
                total_time += i.time;
                print!(" Part 1: ");
                i.display();
            }
            None => println!(" No answer found for part 1."),
        };
        match self.part2(&vec) {
            Some(i) => {
                total_time += i.time;
                print!(" Part 2: ");
                i.display();
            }
            None => println!(" No answer found for part 2."),
        };
        println!("");
        total_time
    }
    #[allow(unused)]
    fn part1(&self, vec: &Vec<String>) -> Option<Answer> {
        None
    }
    #[allow(unused)]
    fn part2(&self, vec: &Vec<String>) -> Option<Answer> {
        None
    }
}
