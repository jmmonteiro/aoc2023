use aoc2023::days;
use std::time::{Duration, Instant};

use aoc2023::utils::structs::Solver;
fn main() {
    let time = Instant::now();
    let mut total_algo_time = Duration::new(0, 0);

    println!("");
    total_algo_time += days::day01::Day.solve("inputs/01/input.txt");
    total_algo_time += days::day02::Day.solve("inputs/02/input.txt");
    total_algo_time += days::day03::Day.solve("inputs/03/input.txt");
    total_algo_time += days::day04::Day.solve("inputs/04/input.txt");
    // total_algo_time += days::day05::Day.solve("inputs/05/input.txt"); // too slow to run
    total_algo_time += days::day06::Day.solve("inputs/06/input.txt");
    total_algo_time += days::day07::Day.solve("inputs/07/input.txt");
    total_algo_time += days::day08::Day.solve("inputs/08/input.txt");
    total_algo_time += days::day09::Day.solve("inputs/09/input.txt");
    total_algo_time += days::day10::Day.solve("inputs/10/input.txt");
    total_algo_time += days::day11::Day.solve("inputs/11/input.txt");
    // total_algo_time += days::day12::Day.solve("inputs/12/input.txt"); // Not done yet
    total_algo_time += days::day13::Day.solve("inputs/13/input.txt");
    total_algo_time += days::day14::Day.solve("inputs/14/input.txt");
    total_algo_time += days::day15::Day.solve("inputs/15/input.txt");

    println!(
        "Total algorithm solve time (excludes data reading time): {} s",
        (total_algo_time.as_nanos() as f32) / (i32::pow(10, 9) as f32)
    );
    println!(
        "Total runtime: {} s\n",
        (time.elapsed().as_nanos() as f32) / (i32::pow(10, 9) as f32)
    );
}
