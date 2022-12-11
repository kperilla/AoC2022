use std::collections::HashSet;
use std::ops;
// use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn new(dir_char: char) -> Self {
        match dir_char {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_pt(&mut self, direction: &Direction, distance: u16) {
        match direction {
            Direction::Up => self.y += distance as i32,
            Direction::Down => self.y -= distance as i32,
            Direction::Left => self.x -= distance as i32,
            Direction::Right => self.x += distance as i32,
        }
    }
    fn is_adjacent(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
    fn get_direction_from_difference(diff: &Point) -> Vec<Direction> {
        let mut dirs = Vec::new();
        if diff.x > 0 {
            dirs.push(Direction::Right);
        } else if diff.x < 0 {
            dirs.push(Direction::Left);
        }
        if diff.y > 0 {
            dirs.push(Direction::Up);
        } else if diff.y < 0 {
            dirs.push(Direction::Down);
        }
        return dirs;
    }
}


impl ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Point) -> Point {
        Point{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

struct MoveInstruction {
    direction: Direction,
    num: u16,
}
impl MoveInstruction {
    fn new(line: String) -> Self {
        if let Ok((dir_char, num)) = scan_fmt!(&line, "{} {}", char, u16) {
            let direction = Direction::new(dir_char);
            return Self{direction, num};
        } else {
            panic!("Couldn't read instruction");
        }
    }
}

pub fn part1(input: String) {
    println!("Part 1");
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut head = Point{x: 0, y: 0};
    let mut tail = Point{x: 0, y: 0};
    visited_points.insert(tail.clone());
    for line in input.lines() {
        let instruction = MoveInstruction::new(line.to_string());
        for _ in 0..instruction.num {
            head.move_pt(&instruction.direction, 1);
            if !head.is_adjacent(&tail) {
                let moves = Point::get_direction_from_difference(&(head - tail));
                for dir in moves {
                    tail.move_pt(&dir, 1);
                }
                visited_points.insert(tail.clone());
            }
        }
    }
    println!("Visited: {}", visited_points.len());
}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string("src/day9/testinput")
    //     .expect("Should have been able to read the file");
    let tail_ix: usize = 9;
    let mut knots: Vec<Point> = Vec::new();
    for _ in 0..=tail_ix {
        knots.push(Point{x: 0, y: 0});
    }
    let mut visited_points: HashSet<Point> = HashSet::new();
    visited_points.insert(knots[tail_ix].clone());
    for line in input.lines() {
        let instruction = MoveInstruction::new(line.to_string());
        for _ in 0..instruction.num {
            knots[0].move_pt(&instruction.direction, 1);
            for i in 1..knots.len() {
                if !knots[i - 1].is_adjacent(&knots[i]) {
                    let moves = Point::get_direction_from_difference(&(knots[i - 1] - knots[i]));
                    for dir in moves {
                        knots[i].move_pt(&dir, 1);
                    }
                    if i == tail_ix {
                        visited_points.insert(knots[i].clone());
                    }
                }
            }
        }
    }
    println!("Visited: {}", visited_points.len());

}
