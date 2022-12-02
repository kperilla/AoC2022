use std::{fs, vec};
use phf::phf_map;

const INPUT_PATH: &str = "src/day2/input";

#[derive(PartialEq, Eq, Clone, PartialOrd)]
pub enum RpsMove {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

#[derive(Clone)]
pub enum RpsResult {
    Win,
    Draw,
    Lose,
}



pub struct RpsRound {
    their_move: RpsMove,
    your_move: RpsMove,
    their_score: u32,
    your_score: u32,
}

impl RpsRound {
    pub fn new(their_letter: &str, your_letter: &str) -> Self {
        let their_move = match their_letter {
            "A" => RpsMove::Rock,
            "B" => RpsMove::Paper,
            "C" => RpsMove::Scissors,
            _ => RpsMove::Invalid,
        };
        let your_move = match your_letter {
            "X" => RpsMove::Rock,
            "Y" => RpsMove::Paper,
            "Z" => RpsMove::Scissors,
            _ => RpsMove::Invalid,
        };
        return Self{their_move, your_move, their_score: 0, your_score: 0};
    }



    fn move_to_score(&self, played_move: &RpsMove) -> u32 {
        match played_move {
            RpsMove::Rock => 1,
            RpsMove::Paper => 2,
            RpsMove::Scissors => 3,
            _ => 0,
        }
    }

    fn result_to_score(&self, result: &RpsResult) -> u32 {
        match result {
            RpsResult::Win => 6,
            RpsResult::Draw => 3,
            RpsResult::Lose => 0,
            _ => 0,
        }
    }

    pub fn score_round(&mut self) -> u32 {
        // Returns your score gained
        self.their_score += self.move_to_score(&self.their_move);
        self.your_score += self.move_to_score(&self.your_move);

        if self.your_move == self.their_move {
            self.their_score += self.result_to_score(&RpsResult::Draw);
            self.your_score += self.result_to_score(&RpsResult::Draw);
        } else if (self.your_move == RpsMove::Rock && self.their_move == RpsMove::Scissors ||
                   self.your_move == RpsMove::Scissors && self.their_move == RpsMove::Paper ||
                   self.your_move == RpsMove::Paper && self.their_move == RpsMove::Rock)
        {
            self.their_score += self.result_to_score(&RpsResult::Lose);
            self.your_score += self.result_to_score(&RpsResult::Win);
        } else {
            self.their_score += self.result_to_score(&RpsResult::Win);
            self.your_score += self.result_to_score(&RpsResult::Lose);
        }
        return self.your_score;
    }
}

pub fn round_from_line_string(line: &str) -> RpsRound {
    let move_strs: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    if move_strs.len() != 2 {
        println!("Something went wrong there should be 2 strs");
    }
    return RpsRound::new(move_strs.first().unwrap(), move_strs.last().unwrap());
}

pub fn part1() {
    println!("Part 1");
    let input = fs::read_to_string(INPUT_PATH)
        .expect("Should have been able to read the file");
    let split_input = input.trim().split("\n");
    let mut your_total_score: u32 = 0;
    for line in split_input {
        let mut round = round_from_line_string(line);
        your_total_score += round.score_round();
    }
    println!("Total score: {}", your_total_score);
}

pub fn part2(){
    println!("Part 2");
}
