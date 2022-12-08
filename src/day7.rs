use std::{collections::HashMap, iter::Peekable, str::FromStr, vec};

use crate::{shared::PuzzleResult, test_solvers};

use self::fs::{Directory, File, Node};

mod fs;

pub fn part1(input: &str) -> PuzzleResult {
    let mut commands = transform_input(input).peekable();

    let dir_sizes = dir_sizes(&mut commands);

    let result = dir_sizes
        .values()
        .filter(|&&dir_size| dir_size <= 100_000)
        .sum::<u32>() as i64;

    result.into()
}

pub fn part2(input: &str) -> PuzzleResult {
    let mut commands = transform_input(input).peekable();

    let dir_sizes = dir_sizes(&mut commands);

    const DISK_SPACE: u32 = 70_000_000;
    const REQUIRED_SPACE: u32 = 30_000_000;

    let disk_usage = dir_sizes["/"];

    let space_left = DISK_SPACE - disk_usage;
    let space_needed = REQUIRED_SPACE - space_left;

    let result = dir_sizes
        .values()
        .copied()
        .filter(|&dir_size| dir_size >= space_needed)
        .min()
        .unwrap() as i64;

    result.into()
}

fn dir_sizes<'a, I: Iterator<Item = &'a str>>(commands: &mut Peekable<I>) -> HashMap<String, u32> {
    let mut path = vec!["/".to_owned()];
    let mut dir_sizes = HashMap::new();
    dir_sizes.insert("/".to_owned(), 0u32);

    while let Some(cmd) = commands.next() {
        match cmd.split(' ').collect::<Vec<_>>().as_slice() {
            ["$", "ls"] => {
                let nodes = ls_result(commands);
                let dirs_paths = dirs_paths(&dir_sizes);
                let current_path = path.join("");

                update_dirs(&mut dir_sizes, &dirs_paths, &current_path, &nodes);
            }
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", cd_path] => {
                path.push(trailing_slash(cd_path));
            }

            _ => panic!("unexpected command"),
        }
    }

    dir_sizes
}

fn ls_result<'a, I: Iterator<Item = &'a str>>(commands: &mut Peekable<I>) -> Vec<Node> {
    let mut result = vec![];

    while is_next_result(commands) {
        let node_info = commands.next().unwrap();
        let node = Node::from_str(node_info).expect("node should be valid");

        result.push(node);
    }

    result
}

fn update_dirs(
    mut dir_sizes: &mut HashMap<String, u32>,
    dirs_paths: &Vec<String>,
    path: &str,
    nodes: &Vec<Node>,
) -> () {
    for node in nodes {
        match node {
            Node::DirectoryNode(dir) => {
                create_dir(&mut dir_sizes, path, dir);
            }
            Node::FileNode(file) => {
                add_file(&mut dir_sizes, &dirs_paths, path, file);
            }
        };
    }
}

fn create_dir(dir_sizes: &mut HashMap<String, u32>, path: &str, dir: &Directory) -> () {
    let dir_path = trailing_slash(&(path.to_owned() + dir.name()));
    dir_sizes.insert(dir_path, 0);
}

fn add_file(
    dir_sizes: &mut HashMap<String, u32>,
    dirs_paths: &Vec<String>,
    path: &str,
    file: &File,
) -> () {
    for dir_path in dirs_paths {
        // update sizes of all parent directories
        if path.starts_with(dir_path) {
            let dir_size = dir_sizes.get_mut(dir_path).unwrap();
            *dir_size += file.size();
        }
    }
}

fn dirs_paths(dir_sizes: &HashMap<String, u32>) -> Vec<String> {
    dir_sizes.keys().cloned().collect()
}

fn is_next_result<'a, I: Iterator<Item = &'a str>>(commands: &mut Peekable<I>) -> bool {
    commands.peek().cloned().map(is_result).unwrap_or(false)
}

fn is_result(cmd: &str) -> bool {
    !is_cmd(cmd)
}

fn is_cmd(cmd: &str) -> bool {
    cmd.starts_with("$")
}

fn trailing_slash(path: &str) -> String {
    path.to_owned() + "/"
}

fn transform_input(input: &str) -> impl Iterator<Item = &str> {
    let mut iter = input.lines().peekable();

    if iter
        .peek()
        .map(|&first_cmd| first_cmd != "$ cd /")
        .unwrap_or(true)
    {
        panic!("first command should be '$ cd /'")
    }

    iter.skip(1)
}

test_solvers!(95437, 24933642);
