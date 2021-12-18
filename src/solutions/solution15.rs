use crate::utils::read_2d_int_array;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn solution15 () {
    let map = read_2d_int_array("src/data/solution15.txt");
    println!("{}", solution15a(&map));
    println!("{}", solution15b(&map));
}

type Map = Vec<Vec<u8>>;

fn solution15a(map: &Map) -> u32 {
    a_star(map, false)
}

fn solution15b(map: &Map) -> u32 {
    a_star(map, true)
}

// A* pathfinding approach to finding the exit
fn a_star(map: &Map, larger_map: bool) -> u32 {
    let map_width = map[0].len();
    let map_height = map.len();

    let goal_coords = if !larger_map {
        (map_width - 1, map_height - 1)
    } else {
        (map_width * 5 - 1, map_height * 5 - 1)
    };

    // Nodes adjacent to those we have visited
    let mut known = Vec::<Node>::new();
    // Nodes we have visited and won't need to consider again
    let mut visited = HashSet::<Node>::new();
    
    let first_node = Node::new((0, 0), 0 as u32, &goal_coords);
    insert_node(&mut known, &first_node);

    while known.len() > 0 {
        let cur_node = known.remove(0);

        // Have we met our goal?
        if node_at_coords(&cur_node, &goal_coords) {
            return cur_node.known_cost;
        }

        for neighbour in get_neighbours(&cur_node, &map, &goal_coords) {
            // If this node is not in the closed list (whether just removed or not)
            if !visited.contains(&neighbour) {
                // Add it to the found list, or revise with a lower cost if possible
                insert_node(&mut known, &neighbour);
            }
        }

        visited.insert(cur_node);
    }

    panic!("Ran out of found nodes!");
}

fn insert_node(list: &mut Vec<Node>, node: &Node) -> () {
    if let Some(found) = get_node(list, node) {
        // If the node is already in the list then lower its cost if necessary
        if node.known_cost < found.known_cost {
            found.known_cost = node.known_cost;
            found.total_cost = node.total_cost;
        }
    } else {
        // Add to list if not previously present
        list.push(node.clone());
    }

    // Re-sort the list, since we may have added a node or revised a cost. Sorting is done
    // by "total" cost, a combination of the known cost to get there and the guess of
    // the remaining distance
    list.sort_by_key(|node| node.total_cost);
}

fn get_node<'a, 'b>(list: &'a mut Vec<Node>, target: &'b Node) -> Option<&'a mut Node> {
    list.iter_mut()
        .find(|node| node.coords == target.coords)
}

fn get_neighbours(node: &Node, map: &Map, goal_coords: &(usize, usize)) -> Vec<Node> {
    let (max_x, max_y) = goal_coords;

    // Orthogonal neighbours on each side
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter()
        .map(|(x_off, y_off): &(i32, i32)| (x_off + node.coords.0 as i32, y_off + node.coords.1 as i32))
        // Neighbour's coordinates must be within the map
        .filter(|&(x, y)| x >= 0 && y >= 0
            && (x <= (*max_x as i32) && y <= (*max_y as i32)))
        .map(|(x, y)| Node::new(
            (x as usize, y as usize),
            node.known_cost + get_coord_cost(map, x as usize, y as usize),
            goal_coords
        ))
        .collect::<Vec<Node>>()
}

fn get_coord_cost(map: &Map, x: usize, y: usize) -> u32 {
    let map_height = map.len();
    let map_width = map[0].len();

    let increase = ((x / map_width) + (y / map_height)) as u32;
    // Wrap around increased risk numbers to the 1-9 range
    (map[y%map_height][x%map_width] as u32 + increase - 1) % 9 + 1
}

fn node_at_coords(node: &Node, coords: &(usize, usize)) -> bool {
    node.coords == *coords
}

// Optimistic heuristic to allow for optimal A* solution
fn manhattan(coords_1: &(usize, usize), coords_2: &(usize, usize)) -> u32 {
    ((coords_1.0 as i32 - coords_2.0 as i32).abs()
    + (coords_1.1 as i32 - coords_2.0 as i32).abs()) as u32
}

#[derive(Clone)]
struct Node {
    coords: (usize, usize),
    known_cost: u32,
    guess_remaining: u32,
    total_cost: u32
}

impl Node {
    fn new(coords: (usize, usize), known_cost: u32, goal_coords: &(usize, usize)) -> Self {
        let guess_remaining = manhattan(&coords, goal_coords);
        Node {
            coords,
            known_cost,
            guess_remaining,
            total_cost: known_cost + guess_remaining
        }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coords.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl Eq for Node {}