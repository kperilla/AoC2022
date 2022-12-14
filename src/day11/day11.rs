// Key lesson: (a % (x * y * z)) % x = a % x AND (a % (x * y * z)) % y = a % y, etc.

use std::collections::VecDeque;

fn parse_starting_items(line: String) -> VecDeque<u64> {
    if !line.starts_with("Starting items: ") {
        println!("{}", line);
        panic!("Bad string");
    }
    let num_str = &line["Starting items: ".len()..];
    let items = num_str.split(", ").map(|x| x.parse::<u64>().unwrap()).collect::<VecDeque<u64>>();
    return items;
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Square,
}

fn parse_operation(line: String) -> (Operation, u32) {
    if !line.starts_with("Operation: new = old ") {
        panic!("Bad string");
    }
    let important_part = &line["Operation: new = old ".len()..];
    let operation = match &important_part[..1] {
        "*" => {
            if important_part.ends_with("old") {
                Operation::Square
            } else {
                Operation::Multiply
            }
        },
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        _ => panic!(),
    };
    match operation {
        Operation::Square => (operation, 0),
        _ => (operation, important_part[2..].parse::<u32>().unwrap()),
    }
}

fn parse_test(lines: Vec<&str>) -> (u32, (usize, usize)) {
    if !lines[0].starts_with("Test: ") || lines.len() != 3 {
        panic!("Bad test string");
    }
    let div_check = lines[0]["Test: divisible by ".len()..].parse::<u32>().unwrap();
    return (div_check, (parse_if_line(lines[1].to_string()), parse_if_line(lines[2].to_string())))
}

fn parse_if_line(line: String) -> usize {
    if line.starts_with("If true") {
        return line["If true: throw to monkey ".len()..].parse::<usize>().unwrap();
    } else if line.starts_with("If false") {
        return line["If false: throw to monkey ".len()..].parse::<usize>().unwrap();
    } else {
        panic!("Bad string");
    }
}


#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    operation_num: u32,
    div_check: u32,
    test_dst: (usize, usize),
    inspect_times: u32,
}

impl Monkey {
    fn new(lines: Vec<&str>) -> Self {
        if lines.len() != 6 {
            panic!("Bad monkey string");
        }
        let items = parse_starting_items(lines[1].trim().to_string());
        let (operation, operation_num) = parse_operation(lines[2].trim().to_string());
        let mut test_vec: Vec<&str> = Vec::new();
        for i in 3..6 {
            test_vec.push(lines[i].trim())
        }
        let (div_check, test_dst) = parse_test(test_vec);
        Self{items, operation, operation_num, div_check, test_dst, inspect_times: 0}
    }
    fn inspect_next(&mut self, divides: bool) -> u64 {
        let mut new_val = match self.operation {
            Operation::Add => self.items[0] + self.operation_num as u64,
            Operation::Multiply => self.items[0] * self.operation_num as u64,
            Operation::Square => self.items[0] * self.items[0],
            Operation::Subtract=> panic!("Not supported"),
        };
        if divides {
            new_val /= 3;
        }
        self.items[0] = new_val;
        self.inspect_times += 1;
        new_val
    }
}

pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string("src/day10/testinput")
    //     .expect("Should have been able to read the file");
    let mut monkeys = input
        .split("\n\n")
        .map(|x| Monkey::new(x.lines().collect::<Vec<&str>>()))
        .collect::<Vec<Monkey>>();
    let mut round = 1;
    loop {
        for i in 0..monkeys.len() {

            while monkeys[i].items.len() > 0 {
                if monkeys[i].inspect_next(true) % (monkeys[i].div_check as u64) == 0 {
                    let dst = monkeys[i].test_dst.0;
                    let item = monkeys[i].items.pop_front().unwrap();
                    monkeys[dst].items.push_back(item);
                } else {
                    let dst = monkeys[i].test_dst.1;
                    let item = monkeys[i].items.pop_front().unwrap();
                    monkeys[dst].items.push_back(item);
                }
            }
        }
        if round == 20 {
            break;
        }
        round += 1;
    }
    let mut inspect_times = monkeys.iter().map(|monkey| monkey.inspect_times).collect::<Vec<u32>>();
    inspect_times.sort();
    let mult = inspect_times[inspect_times.len() - 1] * inspect_times[inspect_times.len() - 2];
    println!("Mult: {}", mult);
}

#[derive(Debug)]
struct ModuloMonkey {
    item_mods: VecDeque<u64>,
    operation: Operation,
    operation_num: u32,
    div_check: u32,
    test_dst: (usize, usize),
    inspect_times: u32,
}
impl ModuloMonkey {
    fn new(lines: Vec<&str>) -> Self {
        if lines.len() != 6 {
            panic!("Bad monkey string");
        }
        let (operation, operation_num) = parse_operation(lines[2].trim().to_string());
        let mut test_vec: Vec<&str> = Vec::new();
        for i in 3..6 {
            test_vec.push(lines[i].trim())
        }
        let (div_check, test_dst) = parse_test(test_vec);
        let item_mods  = parse_starting_items(lines[1].trim().to_string());
        Self{item_mods, operation, operation_num, div_check, test_dst, inspect_times: 0}
    }
    fn inspect_next(&mut self, div_mult: u64) -> u64 {
        let mut new_term = match self.operation {
            Operation::Add => self.item_mods[0] + self.operation_num as u64,
            Operation::Multiply => self.item_mods[0] * self.operation_num as u64,
            Operation::Square => self.item_mods[0] * self.item_mods[0],
            Operation::Subtract=> panic!("Not supported"),
        };
        new_term = new_term % div_mult;
        self.item_mods[0] = new_term;
        // self.items[0] = new_val;
        self.inspect_times += 1;
        new_term
    }
    // fn give(&mut self, other: &mut VecDeque<u64>) {
    //     other.push_back(self.items.pop_front().unwrap());
    // }
}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string("src/day11/testinput")
    //     .expect("Should have been able to read the file");
    let mut monkeys = input.split("\n\n")
        .map(|x| ModuloMonkey::new(x.lines().collect::<Vec<&str>>()))
        .collect::<Vec<ModuloMonkey>>();
    let mut round = 1;
    let mut divisor_mult: u64 = 1;
    for monkey in &monkeys {
        divisor_mult *= monkey.div_check as u64;
    }
    for monkey in &mut monkeys {
        for i in 0..monkey.item_mods.len() {
            monkey.item_mods[i] = monkey.item_mods[i] % divisor_mult as u64;
        }
    }
    loop {
        for i in 0..monkeys.len() {
            // match changes.get_mut(&i) {
            //     None => {},
            //     Some(queue) => {
            //         while queue.len() > 0 {
            //             monkeys[i].items.push_back(queue.pop_front().unwrap());
            //         }
            //     }
            // }

            while monkeys[i].item_mods.len() > 0 {
                if monkeys[i].inspect_next(divisor_mult) % monkeys[i].div_check as u64 == 0 {
                    let dst = monkeys[i].test_dst.0;
                    let item = monkeys[i].item_mods.pop_front().unwrap();
                    monkeys[dst].item_mods.push_back(item);
                    // if !changes.contains_key(&dst) {
                    //     changes.insert(dst, VecDeque::new());
                    // }
                    // let mut change_queue = changes.get_mut(&dst).unwrap();
                    // monkeys[i].give(change_queue);
                    // monkeys[i].give(changes.get_mut(&dst).unwrap());
                    // changes.insert(dst, *change_queue);
                } else {
                    let dst = monkeys[i].test_dst.1;
                    let item = monkeys[i].item_mods.pop_front().unwrap();
                    monkeys[dst].item_mods.push_back(item);
                    // if !changes.contains_key(&dst) {
                    //     changes.insert(dst, VecDeque::new());
                    // }
                    // monkeys[i].give(changes.get_mut(&dst).unwrap());
                }
            }
        }
        if round == 10000 {
            break;
        }
        round += 1;
    }
    let mut inspect_times = monkeys.iter().map(|monkey| monkey.inspect_times).collect::<Vec<u32>>();
    inspect_times.sort();
    let mult: u64 = inspect_times[inspect_times.len() - 1] as u64 * inspect_times[inspect_times.len() - 2] as u64;
    println!("Mult: {}", mult);
}
