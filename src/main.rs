#![allow(unused)]
#[macro_use] extern crate scan_fmt;

use std::io;
use rand::Rng;
use reqwest::blocking::get;
use std::env;
use std::fs;
// use phf::phf_map;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

struct DayEntity {
    part_funcs: Vec<fn(String)>,
    input_path: String,
}

impl DayEntity {
    // const PATH_TEMPLATE: &str = "src/day{}/input";

    pub fn new(day_num: u8) -> Self {
        // let input_path = PATH_TEMPLATE;
        match day_num {
            1 => Self{part_funcs: [day1::day1::part1, day1::day1::part2].to_vec(), input_path: "src/day1/input".to_string()},
            2 => Self{part_funcs: [day2::day2::part1, day2::day2::part2].to_vec(), input_path: "src/day2/input".to_string()},
            3 => Self{part_funcs: [day3::day3::part1, day3::day3::part2].to_vec(), input_path: "src/day3/input".to_string()},
            4 => Self{part_funcs: [day4::day4::part1, day4::day4::part2].to_vec(), input_path: "src/day4/input".to_string()},
            5 => Self{part_funcs: [day5::day5::part1, day5::day5::part2].to_vec(), input_path: "src/day5/input".to_string()},
            6 => Self{part_funcs: [day6::day6::part1, day6::day6::part2].to_vec(), input_path: "src/day6/input".to_string()},
            7 => Self{part_funcs: [day7::day7::part1, day7::day7::part2].to_vec(), input_path: "src/day7/input".to_string()},
            _ => todo!(),
        }
    }
}

// static DAY_FUNCTIONS: phf::Map<&'static str, &'static [fn()]> = phf_map! {
//     "1" => &[day1::day1::part1, day1::day1::part2],
//     "2" => &[day2::day2::part1, day2::day2::part2],
//     "3" => &[day3::day3::part1, day3::day3::part2],
//     "4" => &[day4::day4::part1, day4::day4::part2],
//     "5" => &[day5::day5::part1, day5::day5::part2],
// };

fn get_input(input_path: String) -> String {
    fs::read_to_string(input_path)
        .expect("Should have been able to read the file")
}

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
    let selected_day = DayEntity::new(day_int);
    let func = selected_day.part_funcs[part_index];

    func(get_input(selected_day.input_path));
}
