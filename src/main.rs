#![allow(unused)]
#[macro_use] extern crate scan_fmt;

use std::io;
use rand::Rng;
use std::env;
use phf::phf_map;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

static DAY_FUNCTIONS: phf::Map<&'static str, &'static [fn()]> = phf_map! {
    "1" => &[day1::day1::part1, day1::day1::part2],
    "2" => &[day2::day2::part1, day2::day2::part2],
    "3" => &[day3::day3::part1, day3::day3::part2],
    "4" => &[day4::day4::part1, day4::day4::part2],
    "5" => &[day5::day5::part1, day5::day5::part2],
};

// #[tokio::main]
fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    let day = &args[1];
    let part = &args[2];
    let day_int = day.parse::<u8>().unwrap();
    let part_int = part.parse::<u8>().unwrap();
    let name = String::new();
    println!("{}, {}!", day_int, part_int);
    let part_index = usize::from(part_int - 1);
    let func = DAY_FUNCTIONS[day][part_index];

    func();
}
