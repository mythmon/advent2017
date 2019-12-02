#![feature(try_trait)]

use advent_lib::cases::Puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
// mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        day01::get_puzzles(),
        day02::get_puzzles(),
        day03::get_puzzles(),
        day04::get_puzzles(),
        day05::get_puzzles(),
        day06::get_puzzles(),
        day07::get_puzzles(),
        // TODO accidentally deleted day 8, recreate it
        // Box::new(day08::Part1),
        // Box::new(day08::Part2),
        day09::get_puzzles(),
        day10::get_puzzles(),
        day11::get_puzzles(),
        day12::get_puzzles(),
    ]
    .into_iter()
    .flatten()
    .collect()
}