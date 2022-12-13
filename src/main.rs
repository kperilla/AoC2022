// #![allow(unused)]
#[macro_use] extern crate scan_fmt;

use std::env;
use std::fs;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

struct DayEntity {
    part_funcs: Vec<fn(String)>,
    input_url: String,
}

impl DayEntity {
    const URL_TEMPLATE: &str = "https://adventofcode.com/2022/day/{day_num}/input";

    pub fn new(day_num: u8) -> Self {
        let input_url = DayEntity::URL_TEMPLATE.replace("{day_num}", day_num.to_string().as_str());
        match day_num {
            1 => Self{part_funcs: [day1::day1::part1, day1::day1::part2].to_vec(), input_url},
            2 => Self{part_funcs: [day2::day2::part1, day2::day2::part2].to_vec(), input_url},
            3 => Self{part_funcs: [day3::day3::part1, day3::day3::part2].to_vec(), input_url},
            4 => Self{part_funcs: [day4::day4::part1, day4::day4::part2].to_vec(), input_url},
            5 => Self{part_funcs: [day5::day5::part1, day5::day5::part2].to_vec(), input_url},
            6 => Self{part_funcs: [day6::day6::part1, day6::day6::part2].to_vec(), input_url},
            7 => Self{part_funcs: [day7::day7::part1, day7::day7::part2].to_vec(), input_url},
            8 => Self{part_funcs: [day8::day8::part1, day8::day8::part2].to_vec(), input_url},
            9 => Self{part_funcs: [day9::day9::part1, day9::day9::part2].to_vec(), input_url},
            10 => Self{part_funcs: [day10::day10::part1, day10::day10::part2].to_vec(), input_url},
            11 => Self{part_funcs: [day11::day11::part1, day11::day11::part2].to_vec(), input_url},
            _ => todo!(),
        }
    }
}

// fn get_input(input_path: String) -> String {
//     fs::read_to_string(input_path)
//         .expect("Should have been able to read the file")
// }

async fn get_input_by_http(input_url: String, session_key_path: String) -> Result<String, reqwest::Error> {
    let session_key = "session=".to_string().to_owned() + &fs::read_to_string(session_key_path)
        .expect("Should have been able to read the file").to_owned();
    let input = reqwest::Client::new()
        .get(input_url)
        .header("Cookie", session_key)
        .send()
        .await?
        .text()
        .await?;
    return Ok(input);
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    let day = &args[1];
    let part = &args[2];
    let day_int = day.parse::<u8>().unwrap();
    let part_int = part.parse::<u8>().unwrap();
    println!("{}, {}!", day_int, part_int);
    let part_index = usize::from(part_int - 1);
    let selected_day = DayEntity::new(day_int);
    let func = selected_day.part_funcs[part_index];

    let input = get_input_by_http(selected_day.input_url, "session.key".to_string()).await?;
    let now = Instant::now();
    func(input);
    println!("Time: {} micros", now.elapsed().as_micros());
    Ok(())
}
