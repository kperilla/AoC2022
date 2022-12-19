use std::collections::HashMap;
// use std::fs;
use std::ops::Index;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(pt_str: String) -> Self {
        let xysplit = pt_str
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if xysplit.len() != 2 {
            panic!("Invalid point");
        } else {
            Self {
                x: xysplit[0],
                y: xysplit[1],
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum SimObject {
    Rock,
    Sand,
    Air,
}

struct GridMap {
    grid: HashMap<Point, SimObject>,
    min_x: Option<usize>,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}
impl Index<Point> for GridMap {
    type Output = SimObject;
    fn index(&self, pt: Point) -> &SimObject {
        if let Some(ret_val) = self.grid.get(&pt) {
            return ret_val;
        } else {
            return &SimObject::Air;
        }
    }
}
impl GridMap {
    fn print_map(&self) {
        if let None = self.min_x {
            panic!("Uninitialized map");
        }
        for y in self.min_y..=self.max_y {
            print!("{}\t", y);
            for x in self.min_x.unwrap()..=self.max_x {
                match self[Point { x, y }] {
                    SimObject::Rock => print!("#"),
                    SimObject::Sand => print!("o"),
                    SimObject::Air => print!("."),
                }
            }
            println!();
        }
    }
    fn set_point(&mut self, pt: Point, val: SimObject, update_max: bool) {
        self.grid.insert(pt, val);
        if update_max {
            if let None = self.min_x {
                self.min_x = Some(pt.x);
            } else if pt.x < self.min_x.unwrap() {
                self.min_x = Some(pt.x);
            }
            if pt.x > self.max_x {
                self.max_x = pt.x + 1;
            }
            if pt.y > self.max_y {
                self.max_y = pt.y;
            }
        }
    }
    fn populate_ground_from_line(&mut self, line: &LineSegment, update_max: bool) {
        let (x_range, y_range) = match line.orientation {
            LineOrientation::Horizontal => {
                let start_x: usize;
                let stop_x: usize;
                if line.start.x < line.stop.x {
                    start_x = line.start.x;
                    stop_x = line.stop.x;
                } else {
                    start_x = line.stop.x;
                    stop_x = line.start.x;
                }
                (start_x..=stop_x, line.start.y..=line.start.y)
            }
            LineOrientation::Vertical => {
                let start_y: usize;
                let stop_y: usize;
                if line.start.y < line.stop.y {
                    start_y = line.start.y;
                    stop_y = line.stop.y;
                } else {
                    start_y = line.stop.y;
                    stop_y = line.start.y;
                }
                (line.start.x..=line.start.x, start_y..=stop_y)
            }
        };
        for y in y_range {
            for x in x_range.clone() {
                self.set_point(Point { x, y }, SimObject::Rock, update_max);
            }
        }
    }
    fn drop_sand(&mut self, src_x: usize, is_infi: bool) -> Option<Point> {
        let mut pos = Point { x: src_x, y: 0 };
        if self[pos] == SimObject::Sand {
            return None;
        }
        loop {
            if is_infi {
                if pos.y > self.max_y {
                    return None;
                }
            }
            let down_cand = Point {
                x: pos.x,
                y: pos.y + 1,
            };
            let down_l_cand = Point {
                x: pos.x - 1,
                y: pos.y + 1,
            };
            let down_r_cand = Point {
                x: pos.x + 1,
                y: pos.y + 1,
            };
            if self[down_cand] == SimObject::Air {
                pos = down_cand.clone();
            } else if self[down_l_cand] == SimObject::Air {
                pos = down_l_cand.clone();
            } else if self[down_r_cand] == SimObject::Air {
                pos = down_r_cand.clone();
            } else {
                self.set_point(pos, SimObject::Sand, true);
                return Some(pos);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum LineOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct LineSegment {
    start: Point,
    stop: Point,
    orientation: LineOrientation,
}
impl LineSegment {
    fn string_to_vec(line: String) -> Vec<Self> {
        let point_list = line
            .split(" -> ")
            .map(|x| Point::new(x.to_string()))
            .collect::<Vec<Point>>();
        let mut seg_list = Vec::new();
        for i in 1..point_list.len() {
            let start = point_list[i - 1];
            let stop = point_list[i];
            seg_list.push(Self::new(start, stop));
        }
        return seg_list;
    }
    fn new(start: Point, stop: Point) -> Self {
        if start.x != stop.x && start.y != stop.y {
            panic!("Invalid line: Not horizontal or vertical");
        } else if start.x == stop.x {
            Self {
                start,
                stop,
                orientation: LineOrientation::Vertical,
            }
        } else {
            Self {
                start,
                stop,
                orientation: LineOrientation::Horizontal,
            }
        }
    }
}

pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string("src/day14/testinput")
    //     .expect("Should have been able to read the file");
    let shapes: Vec<Vec<LineSegment>> = input
        .lines()
        .map(|line| LineSegment::string_to_vec(line.to_string()))
        .collect::<Vec<Vec<LineSegment>>>();
    let mut map = GridMap {
        grid: HashMap::new(),
        min_x: None,
        max_x: 0,
        min_y: 0,
        max_y: 0,
    };
    for shape in shapes {
        for seg in shape {
            map.populate_ground_from_line(&seg, true);
        }
    }
    let mut grains = 0;
    while let Some(_) = map.drop_sand(500, true) {
        grains += 1;
    }
    map.print_map();
    println!("Grains: {}", grains);
}

pub fn part2(input: String) {
    println!("Part 2");
    // let input = fs::read_to_string("src/day11/testinput")
    //     .expect("Should have been able to read the file");
    let shapes: Vec<Vec<LineSegment>> = input
        .lines()
        .map(|line| LineSegment::string_to_vec(line.to_string()))
        .collect::<Vec<Vec<LineSegment>>>();
    let mut map = GridMap {
        grid: HashMap::new(),
        min_x: None,
        max_x: 0,
        min_y: 0,
        max_y: 0,
    };
    for shape in shapes {
        for seg in shape {
            map.populate_ground_from_line(&seg, true);
        }
    }
    let ground_left = Point {
        x: 0,
        y: map.max_y + 2,
    };
    let ground_right = Point {
        x: map.max_x * 2,
        y: map.max_y + 2,
    };
    let ground = LineSegment {
        start: ground_left,
        stop: ground_right,
        orientation: LineOrientation::Horizontal,
    };
    map.populate_ground_from_line(&ground, false);
    let mut grains = 0;
    while let Some(_) = map.drop_sand(500, false) {
        grains += 1;
    }
    map.print_map();
    println!("Grains: {}", grains);
}
