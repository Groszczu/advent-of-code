use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, x: i32, y: i32) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Node {
    position: Position,
    height: u8,
}

impl Node {
    pub fn new(position: Position, height: u8) -> Self {
        Self { position, height }
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

#[derive(Debug)]
pub struct Heightmap {
    nodes: Vec<Vec<Node>>,
}

impl Heightmap {
    pub fn new(nodes: Vec<Vec<Node>>) -> Self {
        Self { nodes }
    }

    pub fn get(&self, position: &Position) -> Option<&Node> {
        self.nodes
            .get(position.y as usize)
            .and_then(|row| row.get(position.x as usize))
    }

    pub fn neighbors_leading_to(&self, node: &Node) -> Vec<&Node> {
        Self::neighbors_positions(&node.position)
            .iter()
            .filter_map(|neighbor| self.get(neighbor))
            .filter(|neighbor| neighbor.height + 1 >= node.height)
            .collect()
    }

    fn neighbors_positions(position: &Position) -> [Position; 4] {
        [
            position.add(0, -1),
            position.add(1, 0),
            position.add(0, 1),
            position.add(-1, 0),
        ]
    }

    pub fn distances_to(&self, to_position: Position) -> HashMap<&Node, u32> {
        let mut visited = HashSet::new();
        let mut min_distance_heap = BinaryHeap::new();
        let mut distance_map = HashMap::new();

        let to_node = self
            .get(&to_position)
            .expect("destination node should exist");

        min_distance_heap.push((Reverse(0), to_node));
        distance_map.insert(to_node, 0);

        while let Some((Reverse(distance_to_node), node)) = min_distance_heap.pop() {
            visited.insert(node);

            for neighbor in self.neighbors_leading_to(node) {
                if visited.contains(neighbor) {
                    continue;
                }
                let new_distance = distance_to_node + 1;
                if new_distance < *distance_map.get(neighbor).unwrap_or(&std::u32::MAX) {
                    distance_map.insert(neighbor, new_distance);
                    min_distance_heap.push((Reverse(new_distance), neighbor))
                }
            }
        }

        distance_map
    }

    fn char_to_height(c: char) -> u8 {
        const LOWERCASE_A_ASCII_VALUE: u8 = 97;
        let c = match c {
            'S' => 'a',
            'E' => 'z',
            c => c,
        };

        let ascii_value = c as u8;
        ascii_value - LOWERCASE_A_ASCII_VALUE
    }
}

#[derive(Debug)]
pub struct ParseHeightmapError;

impl FromStr for Heightmap {
    type Err = ParseHeightmapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(column, c)| {
                        Node::new(
                            Position::new(column as i32, row as i32),
                            Self::char_to_height(c),
                        )
                    })
                    .collect()
            })
            .collect();

        Ok(Self::new(nodes))
    }
}
