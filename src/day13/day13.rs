use std::fs;
use std::cmp::Ordering;


#[derive(Debug)]
enum OrderState {
    InOrder,
    OutOfOrder,
    Indeterminate,
}

struct PacketPair {
    left: String,
    right: String,
}

impl PacketPair {
    fn new(pairs_str: &str) -> Self {
        let lines = pairs_str.lines().collect::<Vec<&str>>();
        if lines.len() != 2 {
            panic!();
        } else {
            let left = lines[0].to_string();
            let right = lines[1].to_string();
            Self {left, right}
        }
    }
    fn is_bracketed(list: &Vec<char>) -> bool {
        list[0] == '[' && list[list.len() - 1] == ']'
    }
    fn find_matching_bracket(chars: &Vec<char>, ix_of_first: usize) -> usize {
        let mut depth = 1;
        let mut ix_end = ix_of_first + 1;
        while ix_end < chars.len() {
            match chars[ix_end] {
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => {},
            }
            if depth == 0 {
                return ix_end;
            }
            ix_end += 1;
        }
        panic!("Should have found match");
    }
    fn end_before_condition(left_char: char, right_char: char) -> Option<OrderState> {
        let left_stop = left_char == ']';
        let right_stop = right_char == ']';
        let both_stop = left_stop && right_stop;
        // println!("leftstop: {}, rightstop: {}", left_stop, right_stop);
        if both_stop {
            return Some(OrderState::Indeterminate);
        } else if right_stop {
            return Some(OrderState::OutOfOrder);
        } else if left_stop {
            return Some(OrderState::InOrder);
        }
        return None;
    }
    // fn check_array_condition(left_start: bool, right_start: bool, ) -> Option<OrderState> {

    // }
    fn is_in_order(left: &Vec<char>, right: &Vec<char>) -> OrderState {
        // println!("calling on:");
        // println!("{}", left.iter().collect::<String>());
        // println!("{}", right.iter().collect::<String>());
        // println!();
        if !(PacketPair::is_bracketed(&left) && PacketPair::is_bracketed(&right)) {
            panic!("Not bracketed");
        }
        let mut left_ix: usize = 1;
        let mut right_ix: usize = 1;
        while left_ix < left.len() && right_ix < right.len() {
            // println!("Start of loop: {}, {} of:", left_ix, right_ix);
            // println!("{}", left.iter().collect::<String>());
            // println!("{}", right.iter().collect::<String>());
            if left[left_ix] == ',' {
                left_ix += 1;
            }
            if right[right_ix] == ',' {
                right_ix += 1;
            }
            let left_char: char = left[left_ix];
            let right_char: char = right[right_ix];
            // println!("Left: {}, Right: {}", left_char, right_char);
            // (None is indeterminate)
            // Is either *ix ']'? If right first, return false, if left first return true, if both, return None

            if let Some(in_order) = PacketPair::end_before_condition(left_char, right_char) {
                // println!("returning: {:?}", in_order);
                return in_order;
            }
            // Is either *ix '['? If other is not, call in_order for array of number and other array (if Some, return that)
            let left_start = left_char == '[';
            let right_start = right_char == '[';
            let both_start = left_start && right_start;
            // if left_start || right_start {
            //     let mut left_ender = left_ix;
            //     let mut right_ender = right_ix;
            //     let mut new_left: Vec<char> = Vec::new();
            //     let mut new_right: Vec<char> = Vec::new();
            //     if left_start && right_start {
            //         left_ender = PacketPair::find_matching_bracket(&left, left_ix);
            //         new_left = vec![0 as char; left_ender - left_ix + 1];
            //         new_left.clone_from_slice(&left[left_ix..=left_ender]);
            //         right_ender = PacketPair::find_matching_bracket(&left, right_ix);
            //         new_right = vec![0 as char; right_ender - right_ix + 1];
            //         new_right.clone_from_slice(&right[right_ix..=right_ender]);
            //     }
            //     else if left_start {
            //         left_ender = PacketPair::find_matching_bracket(&left, left_ix);
            //         new_left = vec![0 as char; left_ender - left_ix + 1];
            //         new_left.clone_from_slice(&left[left_ix..=left_ender]);
            //         right_ender = right_ix + 1;
            //         while right_ender < right.len() {
            //             if [',', ']'].contains(&right[right_ender]) {
            //                 break;
            //             }
            //             right_ender += 1;
            //         }
            //         new_right: Vec<char> = "[{}]"
            //             .replace("{}", right[right_ix..right_ender]
            //                 .iter()
            //                 .collect::<String>()
            //                 .as_str())
            //             .chars()
            //             .collect::<Vec<char>>();
            //     } else {
            //         left_ender = PacketPair::find_matching_bracket(&left, left_ix);
            //         new_left = vec![0 as char; left_ender - left_ix + 1];
            //         new_left.clone_from_slice(&left[left_ix..=left_ender]);
            //     }
            //     if let Some(in_order) = PacketPair::check_array_condition(left_char, right_char) {
            //         return in_order;
            //     }
            // }
            // If both are arrays, call in_order for both
            if left_start || right_start {
                if both_start {
                    // println!("Both start");
                    let match_left = PacketPair::find_matching_bracket(&left, left_ix);
                    let match_right = PacketPair::find_matching_bracket(&right, right_ix);
                    let mut new_left: Vec<char> = vec![0 as char; match_left - left_ix + 1];
                    new_left.clone_from_slice(&left[left_ix..=match_left]);
                    let mut new_right: Vec<char> = vec![0 as char; match_right - right_ix + 1];
                    new_right.clone_from_slice(&right[right_ix..=match_right]);
                    match PacketPair::is_in_order(&new_left, &new_right) {
                        OrderState::Indeterminate => {},
                        is_in_order => return is_in_order,
                    }
                    left_ix = match_left + 1;
                    right_ix = match_right + 1;
                // Other wise it's a number until the next ',' or ']'
                } else if left_start {
                    // println!("left start");
                    let match_left = PacketPair::find_matching_bracket(&left, left_ix);
                    let mut new_left: Vec<char> = vec![0 as char; match_left - left_ix + 1];
                    new_left.clone_from_slice(&left[left_ix..=match_left]);
                    let mut number_ender = right_ix + 1;
                    while number_ender < right.len() {
                        if [',', ']'].contains(&right[number_ender]) {
                            break;
                        }
                        number_ender += 1;
                    }
                    let new_right: Vec<char> = "[{}]"
                        .replace("{}", right[right_ix..number_ender]
                            .iter()
                            .collect::<String>()
                            .as_str())
                        .chars()
                        .collect::<Vec<char>>();
                    match PacketPair::is_in_order(&new_left, &new_right) {
                        OrderState::Indeterminate => {},
                        is_in_order => return is_in_order,
                    }
                    // println!("Returning from left_start recurse, indeterminate");
                    // println!("None returned");
                    left_ix = match_left + 1;
                    right_ix = number_ender + 1;
                } else if right_start {
                    let match_right = PacketPair::find_matching_bracket(&right, right_ix);
                    let mut new_right: Vec<char> = vec![0 as char; match_right - right_ix + 1];
                    new_right.clone_from_slice(&right[right_ix..=match_right]);
                    let mut number_ender = left_ix + 1;
                    while number_ender < left.len() {
                        if [',', ']'].contains(&left[number_ender]) {
                            break;
                        }
                        number_ender += 1;
                    }
                    let new_left: Vec<char> = "[{}]"
                        .replace("{}", left[left_ix..number_ender]
                            .iter()
                            .collect::<String>()
                            .as_str())
                        .chars()
                        .collect::<Vec<char>>();
                    // println!("Right was an array, left was number: {}", new_left.iter().collect::<String>());
                    match PacketPair::is_in_order(&new_left, &new_right) {
                        OrderState::Indeterminate => {},
                        is_in_order => return is_in_order,
                    }
                    left_ix = number_ender + 1;
                    right_ix = match_right + 1;
                }
            } else {
                // compare numbers, return true, false, or None
                let mut left_ender = left_ix + 1;
                while left_ender < left.len() {
                    if [',', ']'].contains(&left[left_ender]) {
                        break;
                    }
                    left_ender += 1;
                }
                // println!("Trying to parse: {}, {}, {}", left[left_ix..left_ender].iter().collect::<String>(), left_ix, left_ender);
                let left_num: u32 = left[left_ix..left_ender]
                    .iter().collect::<String>().parse::<u32>().unwrap();
                let mut right_ender = right_ix + 1;
                while right_ender < right.len() {
                    if [',', ']'].contains(&right[right_ender]) {
                        break;
                    }
                    right_ender += 1;
                }
                let right_num: u32 = right[right_ix..right_ender]
                    .iter().collect::<String>().parse::<u32>().unwrap();

                // println!("Comparing {} and {}", left_num, right_num);
                match left_num.cmp(&right_num) {
                    Ordering::Less => return OrderState::InOrder,
                    Ordering::Greater => return OrderState::OutOfOrder,
                    Ordering::Equal => {},
                }
                left_ix = left_ender;
                right_ix = right_ender;
            }
        }
        if left_ix >= left.len() {
            return OrderState::InOrder;
        }
        if right_ix >= right.len() {
            return OrderState::OutOfOrder;
        }

        // repeat at ix = thing after ',' or ']'

        // repeat at ix = thing after ',' or ']'
        return OrderState::Indeterminate;
    }
    fn solve_pair(&self) -> OrderState {
        let left_chars = self.left.chars().collect::<Vec<char>>();
        let right_chars = self.right.chars().collect::<Vec<char>>();
        // println!("From solve_pair, calling on:");
        // println!("{}", left_chars.iter().collect::<String>());
        // println!("{}", right_chars.iter().collect::<String>());
        // println!();
        return PacketPair::is_in_order(&left_chars, &right_chars);

    }
}

pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string("src/day13/testinput")
    //     .expect("Should have been able to read the file");
    let pairs = input
        .split("\n\n")
        .map(|x| PacketPair::new(x))
        .collect::<Vec<PacketPair>>();
    let mut ix_sum = 0;
    for i in 0..pairs.len() {
        match pairs[i].solve_pair() {
            OrderState::InOrder => {
                println!("IN ORDER {}", i);
                ix_sum += i + 1;
            },
            OrderState::OutOfOrder => {
                println!("NOT IN ORDER {}", i);
            },
            _ => panic!("Error"),
        }
    }
    println!("Sum: {}", ix_sum);
}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string("src/day11/testinput")
    //     .expect("Should have been able to read the file");
    let no_spaces = input.replace("\n\n", "\n");
    let mut signals = no_spaces.lines().map(|x| x.to_string()).collect::<Vec<String>>();
    let two_sig = "[[2]]".to_string();
    let six_sig = "[[6]]".to_string();
    signals.push(two_sig.clone());
    signals.push(six_sig.clone());
    signals.sort_by(|str_a, str_b| -> Ordering {
        let pair = PacketPair{left: str_a.clone(), right: str_b.clone()};
        match pair.solve_pair() {
            OrderState::InOrder => Ordering::Less,
            OrderState::OutOfOrder => Ordering::Greater,
            OrderState::Indeterminate => Ordering::Equal,
        }
    });
    let mut two_ix: usize = 0;

    let mut six_ix: usize = 0;
    for i in 0..signals.len() {
        if signals[i].eq(&two_sig) {
            two_ix = i;
            break;
        }
    }
    for i in two_ix..signals.len() {
        if signals[i].eq(&six_sig) {
            six_ix = i;
            break;
        }
    }
    let key = (two_ix + 1) * (six_ix + 1);
    println!("Key: {}", key)
}
