use std::{fs, vec};

const INPUT_PATH: &str = "src/day1/input";


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
    let input = fs::read_to_string(INPUT_PATH)
        .expect("Should have been able to read the file");
    let split_input = input.trim().split("\n\n");
    let list_of_list_of_nums = split_input
        .map(|x| x.split("\n")
                        .map(|x| x.parse::<u32>().unwrap()));
    let list_of_sums = list_of_list_of_nums.map(|x| x.sum::<u32>());
    let mut sum_vector: Vec<u32> = list_of_sums.collect();
    sum_vector.sort();
    let vector_len = sum_vector.len();
    let sum = sum_vector[vector_len - 1] + sum_vector[vector_len - 2] + sum_vector[vector_len - 3];
    println!("Sum of top 3: {}", sum);
}
