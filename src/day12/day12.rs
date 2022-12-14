use std::collections::{HashMap, VecDeque};
use std::ops::Index;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct GridMap {
    grid: Vec<Vec<char>>,
}
impl GridMap {
    fn new(input: String) -> Self {
        let grid = input.lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self { grid }
    }
    fn len(&self) -> usize {
        self.grid.len()
    }
    fn width(&self) -> usize {
        self.grid[0].len()
    }
}
impl Index<Point> for GridMap {
    type Output = char;
    fn index(&self, pt: Point) -> &char {
        &self.grid[pt.y][pt.x]
    }
}

#[derive(Debug)]
struct Graph {
    start: Point,
    end: Option<Point>,
    node_map: HashMap<Point, GraphNode>,
    dist_map: HashMap<Point, u32>,
}
impl Graph {
    fn new(grid_map: &GridMap) -> Self {
        let mut node_map = HashMap::new();
        let mut start = Point{x: 0, y: 0};
        let mut end = Point{x: 0, y: 0};
        let mut dist_map = HashMap::new();
        for y in 0..grid_map.len() {
            for x in 0..grid_map.grid[0].len() {
                let pt = Point{x, y};
                let mut height = grid_map[pt];
                if height == 'S' {
                    start = pt;
                    dist_map.insert(pt, 0);
                    // node.dist = Some(0);
                } else if height == 'E' {
                    end = pt;
                }
                if height == 'S' {
                    height = 'a';
                }
                let mut node = GraphNode{height, children: Vec::new()};
                node.check_adjacent_for_children(pt, &grid_map, false);
                node_map.insert(pt, node);
            }
        }
        Self { start: start, end: Some(end), node_map, dist_map }
    }
    fn new_backwards(grid_map: &GridMap) -> Self {
        let mut node_map = HashMap::new();
        // let mut start = Point{x: 0, y: 0};
        let mut end = Point{x: 0, y: 0};
        let mut dist_map = HashMap::new();
        for y in 0..grid_map.len() {
            for x in 0..grid_map.grid[0].len() {
                let pt = Point{x, y};
                let mut height = grid_map[pt];

                if height == 'S' {
                    // start = pt;
                    // node.dist = Some(0);
                } else if height == 'E' {
                    end = pt;
                    dist_map.insert(pt, 0);
                }
                if height == 'S' {
                    height = 'a';
                }
                let mut node = GraphNode{height, children: Vec::new()};
                node.check_adjacent_for_children(pt, &grid_map, true);
                node_map.insert(pt, node);
            }
        }
        Self { start: end, end: None, node_map, dist_map }
    }
}

#[derive(Debug)]
struct GraphNode {
    height: char,
    children: Vec<Point>,
}
impl GraphNode {
    fn is_viable_path(current_char: char, check_char: char, backwards: bool) -> bool {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let effective_curr = match current_char {
            'S' => alphabet.find('a'),
            'E' => alphabet.find('z'),
            actual => alphabet.find(actual),
        }.unwrap();
        let effective_check = match check_char {
            'S' => alphabet.find('a'),
            'E' => alphabet.find('z'),
            actual => alphabet.find(actual),
        }.unwrap();
        if backwards {
            if effective_curr > 0 {
                effective_check >= effective_curr - 1
            } else {
                true
            }
        } else {
            effective_check <= effective_curr + 1
        }
    }

    fn check_adjacent_for_children(&mut self, pt: Point, grid: &GridMap, backwards: bool) {
        let current_char = grid[pt];
        if pt.y > 0 {
            let check_pt = Point{x: pt.x, y: pt.y - 1};
            let check_char = grid[check_pt];
            if GraphNode::is_viable_path(current_char, check_char, backwards) {
                self.children.push(check_pt);
            }
        }
        if pt.y < grid.len() - 1 {
            let check_pt = Point{x: pt.x, y: pt.y + 1};
            let check_char = grid[check_pt];
            if GraphNode::is_viable_path(current_char, check_char, backwards) {
                self.children.push(check_pt);
            }
        }
        if pt.x > 0 {
            let check_pt = Point{x: pt.x - 1, y: pt.y};
            let check_char = grid[check_pt];
            if GraphNode::is_viable_path(current_char, check_char, backwards) {
                self.children.push(check_pt);
            }
        }
        if pt.x < grid.width() - 1 {
            let check_pt = Point{x: pt.x + 1, y: pt.y};
            let check_char = grid[check_pt];
            if GraphNode::is_viable_path(current_char, check_char, backwards) {
                self.children.push(check_pt);
            }
        }
    }
}

fn bfs_to_end(graph: &mut Graph) {
    let mut visited: HashMap<Point, bool> = HashMap::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(graph.start);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        visited.insert(current, true);
        let curr_node = graph.node_map.get(&current).unwrap();
        let curr_dist = graph.dist_map.get(&current).unwrap().clone();
        for child_pt in &curr_node.children {
            if !visited.contains_key(child_pt) {
                visited.insert(child_pt.clone(), true);
                graph.dist_map.insert(child_pt.clone(), curr_dist + 1);
                queue.push_back(child_pt.clone());
                if child_pt.clone() == graph.end.unwrap() {
                    return;
                }
            }
        }
    }
}

fn bfs_from_end(graph: &mut Graph) -> u32 {
    let mut visited: HashMap<Point, bool> = HashMap::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(graph.start);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        visited.insert(current, true);
        let curr_node = graph.node_map.get(&current).unwrap();
        let curr_dist = graph.dist_map.get(&current).unwrap().clone();
        for child_pt in &curr_node.children {
            if !visited.contains_key(child_pt) {
                visited.insert(child_pt.clone(), true);
                graph.dist_map.insert(child_pt.clone(), curr_dist + 1);
                queue.push_back(child_pt.clone());
                let height = graph.node_map.get(child_pt).unwrap().height;
                if height == 'a' {
                    return curr_dist + 1;
                }
            }
        }
    }
    return 0;
}

pub fn part1(input: String) {
    println!("Part 1");
    // let input = fs::read_to_string("src/day12/testinput")
    //     .expect("Should have been able to read the file");
    let grid_map = GridMap::new(input);
    let mut graph = Graph::new(&grid_map);
    bfs_to_end(&mut graph);
    let end_dist = graph.dist_map.get(&graph.end.unwrap()).unwrap();
    println!("Shortest path: {}", end_dist);
}

pub fn part2(input: String){
    println!("Part 2");
    // let input = fs::read_to_string("src/day11/testinput")
    //     .expect("Should have been able to read the file");
    let grid_map = GridMap::new(input);
    let mut graph = Graph::new_backwards(&grid_map);
    let shortest = bfs_from_end(&mut graph);
    println!("Shortest path: {}", shortest);
}
