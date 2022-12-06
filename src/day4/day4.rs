use std::{fs, vec};
use std::collections::HashSet;

const INPUT_PATH: &str = "src/day4/input";

pub struct ElfRange {
    start: u16,
    end: u16,
}

impl ElfRange {
    pub fn new(range_str: &str) -> Self {
        // println!("new range on {}", range_str);
        let hyphen = range_str.find('-').unwrap();
        let start_str = &range_str[..hyphen];
        let end_str = &range_str[hyphen + 1..];
        return Self{start: start_str.parse::<u16>().unwrap(), end: end_str.parse::<u16>().unwrap()}
    }

    pub fn is_within(&self, other: &ElfRange) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub fn collides(&self, other: &ElfRange) -> bool {
        (self.start <= other.start && other.start <= self.end) || (other.start <= self.start && self.start <= other.end)
    }
}

// You can also use scanfmt!
// Also str.lines()
pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string(INPUT_PATH)
    //     .expect("Should have been able to read the file");
    let split_input = input.trim().split("\n");
    let mut containing_pairs: u32 = 0;
    for line in split_input {
        let elf_parts = line.split(",").collect::<Vec<&str>>();
        let range_1 = ElfRange::new(elf_parts[0]);
        let range_2 = ElfRange::new(elf_parts[1]);
        if range_1.is_within(&range_2) || range_2.is_within(&range_1) {
            containing_pairs += 1;
        }
    }

    println!("Total containing pairs: {}", containing_pairs);
}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string(INPUT_PATH)
    //     .expect("Should have been able to read the file");
    let split_input = input.trim().split("\n");
    let mut colliding_pairs: u32 = 0;
    for line in split_input {
        let elf_parts = line.split(",").collect::<Vec<&str>>();
        let range_1 = ElfRange::new(elf_parts[0]);
        let range_2 = ElfRange::new(elf_parts[1]);
        if range_1.collides(&range_2) {
            colliding_pairs += 1;
        }
    }

    println!("Total colliding pairs: {}", colliding_pairs);
}
