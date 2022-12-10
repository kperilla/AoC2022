// Massive credits to https://sachanganesh.com/programming/graph-tree-traversals-in-rust/
use std::{vec, str};

#[derive(Debug, PartialEq)]
enum EntryType {
    File,
    Directory,
}

pub type TreeIndex = usize;

#[derive(Debug)]
struct EntryNode {
    name: String,
    children: Vec<Option<TreeIndex>>,
    parent: Option<TreeIndex>,
    size: u32,
    entry_type: EntryType,
}


pub struct FilesystemTree {
    arena: Vec<Option<EntryNode>>,
    root: Option<TreeIndex>,
}
impl FilesystemTree {
    fn register_to_arena(&mut self, node: EntryNode) -> TreeIndex {
        let index = self.arena.len();
        self.arena.push(Some(node));
        return index;
    }
    fn node_at(&self, index: TreeIndex) -> Option<&EntryNode> {
        return if let Some(node) = self.arena.get(index) {
            node.as_ref()
        } else {
            None
        }
    }
    fn node_at_mut(&mut self, index: TreeIndex) -> Option<&mut EntryNode> {
        return if let Some(node) = self.arena.get_mut(index) {
            node.as_mut()
        } else {
            None
        }
    }
    pub fn iter(&self) -> PostorderIter {
        PostorderIter::new(self.root, &self)
    }
}
pub struct PostorderIter {
    stack: Vec<TreeIndex>
}

impl PostorderIter {
    pub fn new(root: Option<TreeIndex>, fs: &FilesystemTree) -> Self {
        let mut s1: Vec<TreeIndex> = vec![];
        let mut s2: Vec<TreeIndex> = vec![];
        s1.push(root.unwrap());
        while !s1.is_empty() {
            let node_ix = s1.pop().unwrap();
            s2.push(node_ix);
            let node = fs.node_at(node_ix).unwrap();
            for child_ix in &node.children {
                s1.push(child_ix.unwrap());
            }
        }
        return Self{stack: s2};
    }
    pub fn next(&mut self) -> Option<TreeIndex> {
        self.stack.pop()
    }
}

enum CommandType {
    Cd,
    Ls,
}

struct Command {
    command_type: CommandType,
    target: Option<String>,
}

fn build_tree_from_input(input: String) -> FilesystemTree {
    let mut lines = input.lines();
    let curr_line = lines.next();
    match curr_line {
        Some("$ cd /") => (),
        None => panic!("No start"),
        _ => panic!("Unexpected start"),
    };

    let mut fs = FilesystemTree{arena: Vec::new(), root: None};
    let root_node = EntryNode{name: "/".to_string(), children: Vec::new(), parent: None, size: 0, entry_type: EntryType::Directory};
    let mut curr_command = Command{command_type: CommandType::Cd, target: Some("/".to_string())};
    let mut curr_node = fs.register_to_arena(root_node);
    fs.root = Some(curr_node);
    while let Some(line_str) = lines.next() {
        if line_str.starts_with("$ ") {
            let command_words = line_str[2..].split(" ").collect::<Vec<&str>>();
            match command_words[0] {
                "cd" => {
                    let command_type = CommandType::Cd;
                    let target_str = command_words[1].to_string();
                    if command_words.len() >= 1 {
                        curr_command = Command{command_type, target: Some(target_str.clone())};
                    } else {
                        panic!("Cd without args unsupported");
                    }
                    match target_str.as_str() {
                        ".." => {
                            let parent_node_ix = fs.node_at(curr_node).unwrap().parent.unwrap();
                            curr_node = parent_node_ix;
                        },
                        _ => {
                            let curr_children = &fs.node_at(curr_node).unwrap().children;
                            let mut found_child = false;
                            for child_ix in curr_children {
                                let child_node = fs.node_at(child_ix.unwrap());
                                if child_node.unwrap().name == curr_command.target.clone().unwrap() {
                                    curr_node = child_ix.unwrap();
                                    found_child = true;
                                    break;
                                }
                            }
                            if !found_child {
                                panic!("No child of the name {} found", curr_command.target.clone().unwrap());
                            }

                        }
                    }

                },
                "ls" => {
                    curr_command = Command{command_type: CommandType::Ls, target: None};
                },
                _ => panic!("Unexpected command found"),
            };
        } else {
            match curr_command.command_type {
                CommandType::Ls => {
                    let result_words = line_str.split(" ").collect::<Vec<&str>>();
                    match result_words[0] {
                        "dir" => {
                            // Add a Directory node for currnode
                            let dir_node = EntryNode{name: result_words[1].to_string(),
                                                                 children: Vec::new(), parent: Some(curr_node),
                                                                 size: 0, entry_type: EntryType::Directory};
                            let dir_ix = fs.register_to_arena(dir_node);
                            fs.node_at_mut(curr_node).unwrap().children.push(Some(dir_ix));
                        },
                        s => {
                            if let Ok(file_size) = s.parse::<u32>() {
                                // Add a File node for currnode
                                let file_node = EntryNode{name: result_words[1].to_string(),
                                                                     children: Vec::new(), parent: Some(curr_node),
                                                                     size: file_size, entry_type: EntryType::File};
                                let file_ix = fs.register_to_arena(file_node);
                                fs.node_at_mut(curr_node).unwrap().children.push(Some(file_ix));
                            } else {
                                panic!("Unexpected output");
                            }
                        }
                    }
                },
                _ => panic!("Unsupported"),
            }
        }
    }
    return fs;
}

fn scan_dir_sizes(fs: &mut FilesystemTree) -> Vec<(u32, TreeIndex)> {
    let mut dir_sizes: Vec<(u32, TreeIndex)> = Vec::new();
    let mut tree_iter = fs.iter();
    while let Some(node_ix) = tree_iter.next() {
        let node = fs.node_at(node_ix).unwrap();
        if node.entry_type == EntryType::Directory {
            dir_sizes.push((node.size, node_ix));
        }
        let node_size = fs.node_at(node_ix).unwrap().size;
        if let Some(parent_ix) = node.parent {
            let mut parent = fs.node_at_mut(parent_ix).unwrap();
            parent.size += node_size;
        } else {
            break;
        }

    }
    return dir_sizes;
}

pub fn part1(input: String) {
    println!("Part 1");
    let mut fs = build_tree_from_input(input);
    let dir_sizes = scan_dir_sizes(&mut fs);
    let mut just_sizes = dir_sizes.iter().map(|x| x.0).collect::<Vec<u32>>();
    just_sizes.sort();
    let mut size_sum: u32 = 0;
    for size in just_sizes {
        if size <= 100000 {
            size_sum += size;
        } else {
            break;
        }
    }
    println!("Sizes: {}", size_sum);

}

pub fn part2(input: String){
    println!("Part 2");
    let mut fs = build_tree_from_input(input);
    let dir_sizes = scan_dir_sizes(&mut fs);
    let mut just_sizes = dir_sizes.iter().map(|x| x.0).collect::<Vec<u32>>();
    just_sizes.sort();
    let total_space = 70000000;
    let total_used = just_sizes[just_sizes.len() - 1];
    let space_needed = 30000000;
    let space_to_delete = space_needed - (total_space - total_used);
    let mut size_of_delete: u32 = 0;
    for size in just_sizes {
        if size >= space_to_delete {
            size_of_delete = size;
            break;
        }
    }
    println!("Size: {}", size_of_delete);
}
