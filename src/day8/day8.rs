use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_pt(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

struct Grid {
    grid_map: Vec<Vec<char>>,
    max_width: i32,
    max_height: i32,
}
impl Grid {
    fn new(input: String) -> Self {
        let grid_map: Vec<Vec<char>> = input.lines().collect::<Vec<&str>>().iter().map(|x| x.chars().collect::<Vec<char>>()).collect();
        let max_width = grid_map[0].len() as i32;
        let max_height = grid_map.len() as i32;
        Self {grid_map, max_width, max_height}
    }
    fn check_edge(&self, pt: &Point, direction: &Direction) -> bool {
        match direction {
            Direction::Up => pt.y == 0,
            Direction::Down => pt.y == self.max_height - 1,
            Direction::Left => pt.x == 0,
            Direction::Right => pt.x == self.max_width - 1,
        }
    }

    fn get_visible_from(&self, start_x: i32, start_y: i32, direction: Direction) -> HashSet<Point> {
        let mut visibles: HashSet<Point> = HashSet::new();
        let mut pt = Point{x: start_x, y: start_y};
        pt.move_pt(&direction);
        if pt.x < 0 || pt.y < 0 || pt.x >= self.max_width || pt.y >= self.max_height {
            return HashSet::new();
        }
        let pt_val = self.grid_map[pt.y as usize][pt.x as usize];
        let mut highest_point = pt_val;
        visibles.insert(pt.clone());
        let mut reached_edge = self.check_edge(&pt, &direction);
        while !reached_edge {
            pt.move_pt(&direction);
            let pt_val = self.grid_map[pt.y as usize][pt.x as usize];
            if pt_val > highest_point {
                highest_point = pt_val;
                visibles.insert(pt);
            }
            reached_edge = self.check_edge(&pt, &direction);
        }
        return visibles;
    }

    fn get_visible_from_height(&self, start_x: i32, start_y: i32, direction: Direction, height: u8) -> u32 {
        let mut pt = Point{x: start_x, y: start_y};
        let mut visibles = 0;
        let mut reached_edge = self.check_edge(&pt, &direction);
        while !reached_edge {
            pt.move_pt(&direction);
            visibles += 1;
            let pt_val = self.grid_map[pt.y as usize][pt.x as usize];
            if pt_val >= height as char {
                break;
            }
            reached_edge = self.check_edge(&pt, &direction);
        }
        return visibles;
    }
}



pub fn part1(input: String) {
    println!("Part 1");
    let grid = Grid::new(input);
    let mut visible_points: HashSet<Point> = HashSet::new();
    for x in 0..grid.max_width {
        let d_set = grid.get_visible_from(x, -1, Direction::Down);
        visible_points.extend(d_set);
        let u_set = grid.get_visible_from(x, grid.max_height, Direction::Up);
        visible_points.extend(u_set);
    }
    for y in 0..grid.max_height {
        let r_set = grid.get_visible_from(-1, y, Direction::Right);
        visible_points.extend(r_set);
        let l_set = grid.get_visible_from(grid.max_width, y, Direction::Left);
        visible_points.extend(l_set);
    }
    println!("Num: {}", visible_points.len())
}

pub fn part2(input: String){
    println!("Part 2");
    let grid = Grid::new(input);
    let mut max_score = 0;
    for x in 0..grid.max_width {
        for y in 0..grid.max_height {
            let pt_val = grid.grid_map[y as usize][x as usize] as u8;
            let d_set = grid.get_visible_from_height(x, y, Direction::Down, pt_val);
            let u_set = grid.get_visible_from_height(x, y, Direction::Up, pt_val);
            let r_set = grid.get_visible_from_height(x, y, Direction::Right, pt_val);
            let l_set = grid.get_visible_from_height(x, y, Direction::Left, pt_val);
            let scenic_score = d_set * u_set * r_set * l_set;
            if scenic_score > max_score {
                max_score = scenic_score;
            }
        }
    }
    println!("Score: {}", max_score);
}
