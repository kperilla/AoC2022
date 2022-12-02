// use error_chain::error_chain;
// use std::io::Read;
// use std::error::Error;
use std::fs;

const INPUT_PATH: &str = "src/day1/input";

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

// TODO: I couldn't figure out async stuff so come back when I can
// async fn get_input(part: &str) {
    // match reqwest::get("https://httpbin.org/ip").await {
    //     Ok(mut response) => {
    //         if response.status() == reqwest::StatusCode::OK {
    //             match response.text().await {
    //                 Ok(text) => println!("Response: {}", text),
    //                 Err(_) => println!("Could not read response text")
    //             }
    //         } else {
    //             println!("Request error");
    //         }
    //     }
    //     Err(_) => {
    //         println!("Could not make request")
    //     }
    // }
    // let result = reqwest::get("https://httpbin.org/ip").await;
    // Ok(result);
// }

pub fn part1() {
    println!("Part 1");
    let input = fs::read_to_string(INPUT_PATH)
        .expect("Should have been able to read the file");
    let split_input = input.trim().split("\n\n");
    let list_of_list_of_nums = split_input
        .map(|x| x.split("\n")
                        .map(|x| x.parse::<u32>().unwrap()));
    let list_of_sums = list_of_list_of_nums.map(|x| x.sum::<u32>());
    let max = list_of_sums.max();
    println!("Max: {}", max.unwrap());
}

pub fn part2(){
    println!("Part 2");
}
