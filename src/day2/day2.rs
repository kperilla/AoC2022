use std::{fs, vec};

const INPUT_PATH: &str = "src/day2/input";

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd)]
pub enum RpsMove {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

impl RpsMove {
    fn weak_against(&self) -> RpsMove {
        match self {
            RpsMove::Rock => RpsMove::Paper,
            RpsMove::Paper => RpsMove::Scissors,
            RpsMove::Scissors => RpsMove::Rock,
            RpsMove::Invalid => RpsMove::Invalid,
        }
    }
    fn strong_against(&self) -> RpsMove {
        match self {
            RpsMove::Rock => RpsMove::Scissors,
            RpsMove::Paper => RpsMove::Rock,
            RpsMove::Scissors => RpsMove::Paper,
            RpsMove::Invalid => RpsMove::Invalid,
        }
    }
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

fn parse_their_move(their_letter: &str) -> RpsMove {
    match their_letter {
        "A" => RpsMove::Rock,
        "B" => RpsMove::Paper,
        "C" => RpsMove::Scissors,
        _ => RpsMove::Invalid,
    }
}

fn parse_your_move_part_1(your_letter: &str) -> RpsMove {
    match your_letter {
        "X" => RpsMove::Rock,
        "Y" => RpsMove::Paper,
        "Z" => RpsMove::Scissors,
        _ => RpsMove::Invalid,
    }
}

fn parse_your_move_part_2(your_letter: &str, their_move: &RpsMove) -> RpsMove {
    match your_letter {
        "X" => their_move.strong_against(),
        "Y" => their_move.clone(),
        "Z" => their_move.weak_against(),
        _ => RpsMove::Invalid,
    }
}

fn round_from_line_string_part_1(line: &str) -> RpsRound {
    let move_strs: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    if move_strs.len() != 2 {
        println!("Something went wrong there should be 2 strs");
    }
    let their_move = parse_their_move(move_strs.first().unwrap().as_str());
    let your_move = parse_your_move_part_1(move_strs.last().unwrap().as_str());
    return RpsRound{their_move, your_move, their_score: 0, your_score: 0};
}

fn round_from_line_string_part_2(line: &str) -> RpsRound {
    let move_strs: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    if move_strs.len() != 2 {
        println!("Something went wrong there should be 2 strs");
    }
    let their_move = parse_their_move(move_strs.first().unwrap().as_str());
    let your_move = parse_your_move_part_2(move_strs.last().unwrap().as_str(), &their_move);
    return RpsRound{their_move, your_move, their_score: 0, your_score: 0};
}

pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string(INPUT_PATH)
    //     .expect("Should have been able to read the file");
    let split_input = input.trim().split("\n");
    let mut your_total_score: u32 = 0;
    for line in split_input {
        let mut round = round_from_line_string_part_1(line);
        your_total_score += round.score_round();
    }
    println!("Total score: {}", your_total_score);
}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string(INPUT_PATH)
    //     .expect("Should have been able to read the file");
    let split_input = input.trim().split("\n");
    let mut your_total_score: u32 = 0;
    for line in split_input {
        let mut round = round_from_line_string_part_2(line);
        your_total_score += round.score_round();
    }
    println!("Total score: {}", your_total_score);
}
