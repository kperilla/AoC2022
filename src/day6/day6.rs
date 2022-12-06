use std::{fs, vec, str, marker};
use std::ops::{Index, IndexMut};
use regex::Regex;
use std::collections::HashMap;


fn find_end_of_unique_chars(input: String, marker_len: usize) -> usize {
    let char_list = input.trim().as_bytes();
    let mut first_valid_point = 0;
    for list_ix in (marker_len - 1)..char_list.len() {
        let mut valid = true;
        let mut slice_occurences: HashMap<u8, bool> = HashMap::new();
        for slice_ix in (list_ix - (marker_len - 1))..=list_ix {
            if slice_occurences.contains_key(&(char_list[slice_ix])) {
                valid = false;
                break;
            } else {
                slice_occurences.insert(char_list[slice_ix], true);
            }
        }
        if valid {
            first_valid_point = list_ix + 1;
            break;
        }
    }
    return first_valid_point;
}


pub fn part1(input: String) {
    println!("Part 1");
    println!("First valid point: {}", find_end_of_unique_chars(input, 4));
}

pub fn part2(input: String){
    println!("Part 2");
    println!("First valid point: {}", find_end_of_unique_chars(input, 14));
}
