use std::{fs, vec};
use std::collections::HashSet;

const INPUT_PATH: &str = "src/day3/input";
const ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string(INPUT_PATH)
    //     .expect("Should have been able to read the file");
    let split_input = input.trim().split("\n");
    let mut total_priority: u32 = 0;
    for row in split_input {
        let comp_1 = &row[..row.len()/2];
        let comp_2 = &row[row.len()/2..];
        let mut match_found = false;
        for char_1 in comp_1.chars() {
            for char_2 in comp_2.chars() {
                if char_1 == char_2 {
                    let priority = ALPHA.find(char_1).unwrap() + 1;
                    total_priority += priority as u32;
                    match_found = true;
                    break;
                }
            }
            if match_found {
                break;
            }
        }
    }
    println!("Total priority: {}", total_priority);

}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string(INPUT_PATH)
    //     .expect("Should have been able to read the file");
    let split_input: Vec<&str> = input.trim().split("\n").collect();
    let mut total_priority: u32 = 0;
    let mut ix = 0;
    while ix < split_input.len() - 2 {
        let mut set_1:HashSet<char> = HashSet::new();
        for char in split_input[ix].chars() {
            set_1.insert(char);
        }
        let mut set_2:HashSet<char> = HashSet::new();
        for char in split_input[ix + 1].chars() {
            set_2.insert(char);
        }
        let mut set_3:HashSet<char> = HashSet::new();
        for char in split_input[ix + 2].chars() {
            set_3.insert(char);
        }

        let intersection: HashSet<char> = set_1.intersection(&set_2)
                                               .cloned()
                                               .collect::<HashSet<char>>()
                                               .intersection(&set_3)
                                               .cloned()
                                               .collect();
        if intersection.len() != 1 {
            println!("THIS IS MORE THAN ONE")
        } else {
            let common_char: char = intersection.into_iter().collect::<Vec<char>>()[0];
            let priority = ALPHA.find(common_char).unwrap() + 1;
            total_priority += priority as u32;
            // for c in intersection {
            //     println!("Common char: {}", c)
            // }

        }
        ix += 3
    }
    println!("Total priority: {}", total_priority);
}
