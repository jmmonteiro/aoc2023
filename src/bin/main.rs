use aoc2023::days;
use std::time::{Duration, Instant};

use aoc2023::utils::structs::Solver;
fn main() {
    let time = Instant::now();
    let mut total_algo_time = Duration::new(0, 0);

    println!("");
    total_algo_time += days::day01::Day.solve("inputs/01/input.txt");
    total_algo_time += days::day02::Day.solve("inputs/02/input.txt");

    println!(
        "Total algorithm solve time (excludes data reading time): {} s",
        (total_algo_time.as_nanos() as f32) / (i32::pow(10, 9) as f32)
    );
    println!(
        "Total runtime: {} s\n",
        (time.elapsed().as_nanos() as f32) / (i32::pow(10, 9) as f32)
    );
}
