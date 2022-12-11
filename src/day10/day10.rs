use std::fs;

#[derive(Copy, Clone)]
enum AsmOp {
    Addx,
    Noop,
}

#[derive(Copy, Clone)]
struct AsmInstruction {
    op: AsmOp,
    val: Option<i16>,
    duration: u16,
}
impl AsmInstruction {
    fn new(line: String) -> Self {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "addx" => Self { op: AsmOp::Addx, val: Some(words[1].parse::<i16>().unwrap()), duration: 2 },
            "noop" => Self { op: AsmOp::Noop, val: None, duration: 1 },
            _ => panic!("Unsupported operation"),
        }
    }
}
struct AsmMachine {
    register_x: i16,
}


pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string("src/day10/testinput")
    //     .expect("Should have been able to read the file");
    let mut machine = AsmMachine{register_x: 1};
    let lines = input.lines().collect::<Vec<&str>>();
    println!("Len: {}", lines.len());
    let important_cycles = [20, 60, 100, 140, 180, 220];
    let mut cycle = 1;
    let mut index = 0;
    let mut wait_timer: u16 = 0;
    let mut current_instruction: Option<AsmInstruction> = None;
    let mut signal_strength = 0;
    loop {
        if wait_timer == 0 {
            match current_instruction {
                Some(instruction) => {
                    match instruction.op {
                        AsmOp::Addx => {
                            machine.register_x += instruction.val.unwrap()
                        },
                        _ => (),
                    }
                    if index >= lines.len() {
                        break;
                    }
                },
                None => (),
            }
            current_instruction = Some(AsmInstruction::new(lines[index].to_string()));
            index += 1;
            wait_timer += current_instruction.unwrap().duration;
        }
        wait_timer -= 1;
        if important_cycles.contains(&cycle) {
            signal_strength += cycle * machine.register_x;
            println!("Cycle {}, Register: {}", cycle, &machine.register_x);
        }
        cycle += 1;
    }
    println!("Signal: {}", signal_strength);
}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string("src/day10/testinput")
    //     .expect("Should have been able to read the file");
    let mut machine = AsmMachine{register_x: 1};
    let lines = input.lines().collect::<Vec<&str>>();
    let mut cycle = 1;
    let mut index = 0;
    let mut wait_timer: u16 = 0;
    let mut current_instruction: Option<AsmInstruction> = None;
    let screen_width = 40;
    loop {

        if wait_timer == 0 {
            match current_instruction {
                Some(instruction) => {
                    match instruction.op {
                        AsmOp::Addx => {
                            machine.register_x += instruction.val.unwrap()
                        },
                        _ => (),
                    }
                    if index >= lines.len() {
                        break;
                    }
                },
                None => (),
            }
            current_instruction = Some(AsmInstruction::new(lines[index].to_string()));
            index += 1;
            wait_timer += current_instruction.unwrap().duration;
        }
        wait_timer -= 1;
        if (((cycle - 1) % screen_width) - machine.register_x).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        if cycle % 40 == 0 {
            println!();
        }
        cycle += 1;
    }

}
