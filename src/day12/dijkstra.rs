use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn nodes(&self) -> Vec<&Node> {
        self.nodes.iter().flatten().collect()
    }

    pub fn neighbors_leading_to(&self, node: &Node) -> Vec<&Node> {
        Self::neighbors_positions(&node.position)
            .iter()
            .filter_map(|neighbor| self.get(neighbor))
            .filter(|neighbor| 1 + neighbor.height >= node.height)
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

    pub fn distances_to(&self, to: Position) -> HashMap<&Node, u32> {
        let mut distance_from_start: HashMap<&Node, u32> = HashMap::new();
        let mut unvisited: HashSet<&Node> = HashSet::new();

        let nodes = self.nodes();
        unvisited.extend(nodes.iter());
        distance_from_start.extend(nodes.iter().map(|&node| (node, std::u32::MAX)));

        let start_node = self.get(&to).unwrap();

        distance_from_start.insert(start_node, 0);

        while !unvisited.is_empty() {
            let node = unvisited
                .iter()
                .min_by(|node_a, node_b| {
                    distance_from_start[*node_a].cmp(&distance_from_start[*node_b])
                })
                .cloned()
                .unwrap();

            if distance_from_start[node] == std::u32::MAX {
                break;
            }

            unvisited.remove(node);

            for neighbor in self.neighbors_leading_to(node) {
                let new_distance = distance_from_start[node] + 1;
                if new_distance < *distance_from_start.get(neighbor).unwrap_or(&std::u32::MAX) {
                    distance_from_start.insert(neighbor, new_distance);
                }
            }
        }

        distance_from_start
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
