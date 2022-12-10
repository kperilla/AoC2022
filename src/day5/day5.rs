use regex::Regex;

pub struct MoveInstruction {
    num: u16,
    src: u16,
    dst: u16,
}

impl MoveInstruction {
    pub fn new(instruction_str: &str) -> Self {
        if let Ok((num, src, dst)) = scan_fmt!(instruction_str, "move {} from {} to {}", u16, u16, u16) {
            return Self{num, src, dst};
        } else {
            println!("Error reading instruction");
            return Self{num:0, src:0, dst:0};
        }
    }
}

pub struct CargoStacks {
    stack_list: Vec<Vec<char>>,
}

impl CargoStacks {
    pub fn new(init_state_str: &str) -> Self {
        let lines = init_state_str.lines().collect::<Vec<&str>>();
        let num_line = lines[lines.len() - 1];
        let mut stack_list = Self::stack_list_init(num_line);
        let row_re = Regex::new(r"\[([A-Z])\]").unwrap();
        for row_ix in (0..=lines.len() - 2).rev() {
            let char_matches = row_re.find_iter(lines[row_ix]);
            for mat in char_matches {
                let stack_num = mat.start() / 4;
                let char_to_add = lines[row_ix].chars().collect::<Vec<char>>()[mat.start() + 1];
                stack_list[stack_num].push(char_to_add);
            }
        }
        return Self{stack_list};
    }

    fn stack_list_init(num_line: &str) -> Vec<Vec<char>> {
        let re = Regex::new(r" (\d+) ").unwrap();
        let matches = re.find_iter(num_line);
        let columns = matches.count();
        let mut stack_list: Vec<Vec<char>> = Vec::with_capacity(columns);
        for _ in 0..columns {
            stack_list.push(Vec::new());
        }
        return stack_list;
    }

    pub fn execute_instruction_single(&mut self, instruction: MoveInstruction) {
        let mut i: u16 = 0;
        while i < instruction.num {
            let to_move = self.stack_list[instruction.src as usize - 1].pop().unwrap();
            self.stack_list[instruction.dst as usize - 1].push(to_move);
            i += 1;
        }
    }

    pub fn execute_instruction_multiple(&mut self, instruction: MoveInstruction) {
        let mut intermediate_stack: Vec<char> = Vec::new();
        for _ in 0..instruction.num {
            let to_move = self.stack_list[instruction.src as usize - 1].pop().unwrap();
            intermediate_stack.push(to_move);
        }
        while !intermediate_stack.is_empty() {
            self.stack_list[instruction.dst as usize - 1].push(intermediate_stack.pop().unwrap());
        }
    }

    pub fn print_diagram(&self) {
        let tallest = self.stack_list.iter().map(|x| x.len()).max().unwrap();
        for height in (0..tallest).rev() {
            for stack in self.stack_list.iter() {
                if height < stack.len() {
                    print!("[{}] ", stack[height]);
                } else {
                    print!("    ");
                }
            }
            println!();
        }
        for stack_ix in 0..self.stack_list.len() {
            print!(" {}  ", stack_ix + 1);
        }
        println!();
    }

    pub fn tops(&self) -> String {
        let mut top_vec: Vec<char> = Vec::new();
        for stack in self.stack_list.iter() {
            top_vec.push(stack.last().unwrap().clone());
        }
        return top_vec.into_iter().collect::<String>();
    }

    pub fn print_tops(&self) {
        let mut top_vec: Vec<char> = Vec::new();
        for stack in self.stack_list.iter() {
            top_vec.push(stack.last().unwrap().clone());
        }
        println!("{}",top_vec.into_iter().collect::<String>().as_str());
    }
}

pub fn part1(input: String) {
    println!("Part 1");
    let split_input = input.split("\n\n").collect::<Vec<&str>>();
    let init_state_drawing = split_input[0];
    let instructions = split_input[1].lines();
    let mut stacks = CargoStacks::new(init_state_drawing);
    stacks.print_diagram();
    for instruction_line in instructions {
        let instruction = MoveInstruction::new(instruction_line);
        stacks.execute_instruction_single(instruction);
    }
    stacks.print_diagram();
    print!("Tops: ");
    stacks.print_tops();
}

pub fn part2(input: String){
    println!("Part 2");
    let split_input = input.split("\n\n").collect::<Vec<&str>>();
    let init_state_drawing = split_input[0];
    let instructions = split_input[1].lines();
    let mut stacks = CargoStacks::new(init_state_drawing);
    stacks.print_diagram();
    for instruction_line in instructions {
        let instruction = MoveInstruction::new(instruction_line);
        stacks.execute_instruction_multiple(instruction);
    }
    stacks.print_diagram();
    println!("Tops: {}", stacks.tops());
}
