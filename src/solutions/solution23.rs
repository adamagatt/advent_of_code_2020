use lazy_static::lazy_static;
use itertools::iproduct;
use permute::permutations_of;
use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::hash::Hash;
use std::fmt::Debug;

pub fn solution23 () {
    // println!("{}", solution23a());
    println!("{}", solution23b());
}

fn solution23a() -> u32 {
    graph_search(&INITIAL_A, &GOAL_A, &CONNECTIONS_A)
}

fn solution23b() -> u32 {
    graph_search(&INITIAL_B, &GOAL_B, &CONNECTIONS_B)
}

fn graph_search<const T: usize>(initial_state: &State<T>, goal_state: &State<T>, connection_map: &ConnectionMap) -> u32 {
    // States adjacent to those we have visited
    let mut known = Vec::<SearchNode<T>>::new();
    // States we have visited and won't need to consider again
    let mut visited = HashSet::<State<T>>::new();

    // First node to expand is initial state with no cost
    known.push(
        SearchNode {
            state: initial_state.clone(), 
            current_cost: 0,
            total_cost_estimate: estimate_remaining_cost(initial_state, goal_state)
        }
    );

    let mut vis_count = 0;
    let mut best_remaining = 999999999;

    while !known.is_empty() {
        let next_node = known.remove(0);
        visited.insert(next_node.state.clone());

        if (visited.len() - vis_count) >= 1000 {
            vis_count = visited.len();
            println!("{}", vis_count);
        }
        if (next_node.total_cost_estimate-next_node.current_cost) < best_remaining {
            best_remaining = next_node.total_cost_estimate-next_node.current_cost;
            println!("NEW BEST: {}", best_remaining);
            dbg!(&next_node.state);
        }


        if next_node.state.eq(goal_state) { // .eq() required instead of == due to lazy_static goal state
            println!("Solution found after {} nodes", visited.len());
            return next_node.current_cost;
        } else {       
            for discovered in find_next_search_nodes(&next_node, goal_state, connection_map) {
                if !visited.contains(&discovered.state) {
                    upsert_node(&mut known, &discovered);
                }
            }
        }
    }
    panic!("No solution found!");
}

fn upsert_node<const T: usize>(list: &mut Vec<SearchNode<T>>, new_node: &SearchNode<T>) {
    let current_node_idx =
        if let Some(found_idx) = find_node_idx(list, new_node) {
            // If this node is already in our search vector then see if its cost might be revised
            let found = list.get_mut(found_idx).unwrap();
            // If the current cost is lower then we have nothing to do
            if found.current_cost <= new_node.current_cost {
                return;
            }

            // We revise the current cost and return the idx of this found node
            found.current_cost = new_node.current_cost;
            found.total_cost_estimate = new_node.total_cost_estimate;

            found_idx
        } else {
            // If the node isn't in the vector we push it on the end and return that last idx
            list.push(new_node.clone());
            list.len()-1
        };

    // We can now find the idx of where the node should be sorted to and rotate it to that position
    // NOTE 1: Whether Result is Ok or Error doesn't matter, only the difference between if another node
    // was already identified with this cost estimate or if this is the first node with that estimate
    // NOTE 2: Binary Search is only valid for sorted lists, and only the list up until the inserted/revised
    // node is guaranteed to be sorted. This is fine for our purposes as the node's sorted position
    // can only be lower or equal to currently, as it is either added to the end of the list or its
    // total estimate has been revised to a lower value.
    let new_idx = list[..current_node_idx].binary_search_by_key(&new_node.total_cost_estimate, |x| x.total_cost_estimate)
        .into_ok_or_err();
    list[new_idx..=current_node_idx].rotate_right(1);
}

fn find_node_idx<const T: usize>(list: &mut [SearchNode<T>], target: &SearchNode<T>) -> Option<usize> {
    list.iter_mut()
        .position(|node| search_nodes_equal(node, target))
}

type Coord = (u32, u32); // (y, x)
type CoordSet <const T: usize> = [Coord; T];
type ConnectionMap = HashMap<Coord, Vec<(Coord, u32)>>;

#[derive (Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum AmphipodType {
    A,
    B,
    C,
    D
}

#[derive (Clone, Eq)]
struct State<const T: usize> {
    a: CoordSet<T>,
    b: CoordSet<T>,
    c: CoordSet<T>,
    d: CoordSet<T>
}

impl <const T: usize> State<T> {
    fn amphipods_of_type(&self, amp_type: AmphipodType) -> CoordSet<T> {
        match amp_type {
            AmphipodType::A => self.a,
            AmphipodType::B => self.b,
            AmphipodType::C => self.c,
            AmphipodType::D => self.d
        }
    }
}

impl <const T: usize> Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "[{:?} - {:?} - {:?} - {:?}]", self.amphipods_of_type(AmphipodType::A), self.amphipods_of_type(AmphipodType::B), self.amphipods_of_type(AmphipodType::C), self.amphipods_of_type(AmphipodType::D))
    }
}

// Custom Hash needed to ensure states match in the HashMap even if both Amphipods
// of the same type are swapped
impl <const T: usize> Hash for State<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        each_amp_type().for_each( |amp_type| {
            let mut sorted_locs = self.amphipods_of_type(amp_type);
            sorted_locs.sort();
            sorted_locs.hash(state);
        });
    }
}

impl <const T: usize> PartialEq for State<T> {
    fn eq(&self, other: &Self) -> bool {
        each_amp_type().all(|amp_type|
            coord_sets_equal(&self.amphipods_of_type(amp_type), &other.amphipods_of_type(amp_type))
        )
    }
}

#[derive (Clone)]
struct SearchNode<const T: usize> {
    state: State<T>,
    current_cost: u32,
    total_cost_estimate: u32
}

fn coord_sets_equal<const T: usize>(left: &CoordSet<T>, right: &CoordSet<T>) -> bool {
    // Sorted to allow equality regardless of order of elements    
    let mut left_sorted = *left; left_sorted.sort();
    let mut right_sorted = *right; right_sorted.sort();
    left_sorted == right_sorted
}

// Nodes in the graph search are considered the same if they have matching states, ignoring costs
fn search_nodes_equal<const T: usize>(left: &SearchNode<T>, right: &SearchNode<T>) -> bool {
    left.state == right.state
}

fn each_amp_type() -> impl Iterator<Item=AmphipodType> {
    IntoIterator::into_iter([
        AmphipodType::A,
        AmphipodType::B,
        AmphipodType::C,
        AmphipodType::D
    ])
}

// !!!
fn each_amphipod<const T: usize>(state: &State<T>) -> Vec<(AmphipodType, Coord, Vec<Coord>)> {
    let state_clone = state.clone();
    each_amp_type().map(
        move |amp_type| (amp_type, state_clone.amphipods_of_type(amp_type))
    )
    // Check both Amphipods of the type
    .flat_map(|(amp_type, amphipods)| {
        match T {
            2 => vec!(
                (amp_type, amphipods[0], vec!(amphipods[1])),
                (amp_type, amphipods[1], vec!(amphipods[0]))
            ),
            4 => vec!(
                (amp_type, amphipods[0], vec!(amphipods[1], amphipods[2], amphipods[3])),
                (amp_type, amphipods[1], vec!(amphipods[0], amphipods[2], amphipods[3])),
                (amp_type, amphipods[2], vec!(amphipods[0], amphipods[1], amphipods[3])),
                (amp_type, amphipods[3], vec!(amphipods[0], amphipods[1], amphipods[2]))
            ),
            _ => unreachable!()
        }
    })
    .collect()
}

fn find_next_search_nodes<const T: usize>(current: &SearchNode<T>, goal_state: &State<T>, connection_map: &ConnectionMap) -> Vec<SearchNode<T>> {
    // Any of the Amphipods can attempt to move a space
    each_amphipod(&current.state).iter()
    // Check each possible destination
    .flat_map(|(amp_type, amphipod, other_amphipods_of_type)| {
        connection_map.get(amphipod).unwrap().iter()
            // Can't move if another amphipod is between the current location and destination
            .filter(move |(new_loc, _)| unblocked_by_other_amphipods(&current.state, *amphipod, new_loc))
            // Won't move into a room unless it is our destination and has no amphipods of other types in it
            .filter(move |(new_loc, _)| only_move_to_room_if_valid(&current.state, *amp_type, new_loc))
            // Create new search node with state for moved amphipod
            .map(move |&(new_loc, distance)| {
                let new_state = state_with_moved_location(new_loc, other_amphipods_of_type, &current.state, *amp_type);
                search_node_for_state(distance, *amp_type, new_state, goal_state, current.current_cost)
            })
    })
    .collect()
}

fn search_node_for_state<const T: usize>(distance: u32, amp_type: AmphipodType, new_state: State<T>, goal_state: &State<T>, previous_cost: u32) -> SearchNode<T> {
    let additional_cost = distance * MOVEMENT_COSTS[&amp_type];
    let remaining_cost = estimate_remaining_cost(&new_state, goal_state);
    SearchNode {
        state: new_state,
        current_cost: previous_cost + additional_cost,
        total_cost_estimate: previous_cost + additional_cost + remaining_cost
    }
}

fn state_with_moved_location<const T: usize>(new_loc: (u32, u32), other_amphipods_of_type: &[(u32, u32)], old_state: &State<T>, amp_type: AmphipodType) -> State<T> {
    let mut new_amphipod_locations = other_amphipods_of_type.to_vec();
    new_amphipod_locations.push(new_loc);
    new_amphipod_locations.sort();
    let mut new_state = old_state.clone();
    match amp_type {
        AmphipodType::A => new_state.a = new_amphipod_locations.try_into().unwrap(),
        AmphipodType::B => new_state.b = new_amphipod_locations.try_into().unwrap(),
        AmphipodType::C => new_state.c = new_amphipod_locations.try_into().unwrap(),
        AmphipodType::D => new_state.d = new_amphipod_locations.try_into().unwrap()
    };
    new_state
}

fn unblocked_by_other_amphipods<const T: usize>(state: &State<T>, amphipod: (u32, u32), dest: &(u32, u32)) -> bool {
    each_amphipod(state).iter().all(|&(_, other_amphipod, _)|
        other_amphipod == amphipod ||
        !blocks_path(&amphipod, dest, &other_amphipod)
    )
}

fn only_move_to_room_if_valid<const T: usize>(state: &State<T>, amp_type: AmphipodType, dest: &(u32, u32)) -> bool {
    // Early success if we are not moving to a room
    if dest.0 == 0 {
        return true;
    }
    let valid_room_idx = match amp_type {
        AmphipodType::A => 2,
        AmphipodType::B => 4,
        AmphipodType::C => 6,
        AmphipodType::D => 8
    };
    // Destination is a valid room for our type
    dest.1 == valid_room_idx &&
    // No amphipods of the wrong type are in our room
    each_amphipod(state).iter().all(|&(other_amp_type, loc, _)|
        other_amp_type == amp_type || loc.1 != dest.1
    )
}

fn estimate_remaining_cost<const T: usize>(state: &State<T>, goal_state: &State<T>) -> u32 {
    // Optimistic estimate of the movement cost required to get both amphipods of each colour to
    // their goals, ignoring all obstacles
    each_amp_type().map(|amp_type|
        (state.amphipods_of_type(amp_type), goal_state.amphipods_of_type(amp_type), MOVEMENT_COSTS[&amp_type])
    )
    .map(|(current_locs, goal_locs, movement_cost)|
        // Compare all alternatives for matching amphipods to goal spaces
        permutations_of(&current_locs)
            .map(|permutation|
                permutation.zip(goal_locs.iter())
                    .map(|(amp_loc, goal_loc)| walk_distance(amp_loc, goal_loc))
                    .sum::<u32>()
            )
            // We want the shortest distance for matching of amphipods to goals, and then
            // multiply by movement cost
            .min().unwrap() * movement_cost
    )
    .sum()
}

// Distance to walk from coord A to B, ignoring any obstacles. Calculated as horizontal offset
// plus distance to walk to and from corridor if needed
fn walk_distance(a: &Coord, b: &Coord) -> u32 {
    // If on the same column then only vertical offset needed
    if a.1 == b.1 {
        a.0.abs_diff(b.0)
    } else {
        // Otherwise we want the horizontal offset plus distance to walk
        // to/from the corridor
        a.1.abs_diff(b.1) + a.0 + b.0
    }
}

fn blocks_path(start: &Coord, dest: &Coord, obstacle: &Coord) -> bool {
    if start.1 == dest.1 {
        // Lined up vertically means they are within a room
        let (lower, higher) = (min(start.0, dest.0), max(start.0, dest.0));
        (obstacle.1 == start.1) && ((lower..=higher).contains(&obstacle.0))
    } else {
        // Otherwise they must traverse via the hallway and might be blocked there
        let (lower, higher) = (min(start.1, dest.1), max(start.1, dest.1));
        // Between (inclusive) x-coordinates of start and dest
        (lower..=higher).contains(&obstacle.1) && (
            // Is in the hallway and thus blocks them
            (obstacle.0 == 0) ||
            // Is in the room with a y-coord at least as shallow as the destination  
            (obstacle.1 == start.1 && obstacle.0 <= start.0) ||
            (obstacle.1 == dest.1 && obstacle.0 <= dest.0)
        )
    }
}

// ,// Precalculate connectivity and costs for moving from a room into the hallway and vice
// versa
fn make_connection_map(hallway_spaces: &HashSet<Coord>, room_spaces: &HashMap<Coord, AmphipodType>) -> ConnectionMap { 
    iproduct!(
        room_spaces.iter().map(|(coord, _)| coord),
        hallway_spaces.iter()
    )
    // Get entries for both directions
    .flat_map(|(start, dest)| {
        let cost = walk_distance(start, dest);
        vec!((*start, *dest, cost), (*dest, *start, cost))
    })
    .fold(HashMap::new(),
        |mut map, (start, dest, cost)| {
            map.entry(start)
                .or_insert(vec!())
                .push((dest, cost));
        map
    })
}

// Map is hard-coded and unchanging and so we can pre-calculate our
// connectivity map, initial and goal states
// #############
// #...........#
// ###B#D#C#A###
//   #C#D#B#A#
//   #########

lazy_static! {
    static ref HALLWAY_STOPS: HashSet<Coord> = HashSet::from([
        (0, 0), (0, 1), (0, 3), (0 ,5), (0, 7), (0, 9), (0, 10)
    ]);

    static ref ROOMS_A: HashMap<Coord, AmphipodType> =
        [(AmphipodType::A, 2), (AmphipodType::B, 4), (AmphipodType::C, 6), (AmphipodType::D, 8)].iter()
        .flat_map(|&(amp_type, x_coord)| (1..=2).map(move |y_coord| ((y_coord, x_coord), amp_type)))
        .collect::<HashMap<Coord, AmphipodType>>();

    static ref CONNECTIONS_A: ConnectionMap = make_connection_map(&HALLWAY_STOPS, &ROOMS_A);

    static ref INITIAL_A: State<2> = State {
        a: [(1, 8), (2, 8)],
        b: [(1, 2), (2, 6)],
        c: [(1, 6), (2, 2)],
        d: [(1, 4), (2, 4)]
    };

    static ref GOAL_A: State<2> = State {
        a: [(1, 2), (2, 2)],
        b: [(1, 4), (2, 4)],
        c: [(1, 6), (2, 6)],
        d: [(1, 8), (2, 8)]
    };

    static ref ROOMS_B: HashMap<Coord, AmphipodType> =
        [(AmphipodType::A, 2), (AmphipodType::B, 4), (AmphipodType::C, 6), (AmphipodType::D, 8)].iter()
        .flat_map(|&(amp_type, x_coord)| (1..=4).map(move |y_coord| ((y_coord, x_coord), amp_type)))
        .collect::<HashMap<Coord, AmphipodType>>();

    static ref CONNECTIONS_B: ConnectionMap = make_connection_map(&HALLWAY_STOPS, &ROOMS_B);

    static ref INITIAL_B: State<4> = State {
        a: [(1, 8), (2, 8), (3, 6), (4, 8)],
        b: [(1, 2), (2, 6), (3, 4), (4, 6)],
        c: [(1, 6), (2, 4), (3, 8), (4, 2)],
        d: [(1, 4), (2, 2), (3, 2), (4, 4)]
    };

    static ref GOAL_B: State<4> = State {
        a: [(1, 2), (2, 2), (3, 2), (4, 2)],
        b: [(1, 4), (2, 4), (3, 4), (4, 4)],
        c: [(1, 6), (2, 6), (3, 6), (4, 6)],
        d: [(1, 8), (2, 8), (3, 8), (4, 8)]
    };

    static ref MOVEMENT_COSTS: HashMap::<AmphipodType, u32> = HashMap::from([
        (AmphipodType::A, 1),
        (AmphipodType::B, 10),
        (AmphipodType::C, 100),
        (AmphipodType::D, 1000)
    ]);
}