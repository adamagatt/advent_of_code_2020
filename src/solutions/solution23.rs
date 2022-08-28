use lazy_static::lazy_static;
use itertools::iproduct;
use std::cmp::{min, max};
use std::collections::{HashMap, HashSet, BTreeMap};
use std::hash::Hash;
use std::fmt::Debug;

pub fn solution23 () {
    println!("{}", solution23a());
    // println!("{}", solution23b());
}

fn solution23a() -> u32 {
    // States adjacent to those we have visited
    let mut known = Vec::<SearchNode>::new();
    // States we have visited and won't need to consider again
    let mut visited = HashSet::<State>::new();

    // First node to expand is initial state with no cost
    known.push(
        SearchNode {
            state: INITIAL.clone(), 
            current_cost: 0,
            total_cost_estimate: estimate_remaining_cost(&INITIAL)
        }
    );

    let mut vis_count = 0;
    let mut best_remaining = 999999999;

    while !known.is_empty() {
        let next_node = known.remove(0);
        visited.insert(next_node.state.clone());

        // dbg!(known.len());
        // dbg!(visited.len());
        // dbg!(&next_node.state, &next_node.current_cost, &next_node.total_cost_estimate);

        if (visited.len() - vis_count) >= 1000 {
            vis_count = visited.len();
            println!("{}", vis_count);
        }
        if (next_node.total_cost_estimate-next_node.current_cost) < best_remaining {
            best_remaining = next_node.total_cost_estimate-next_node.current_cost;
            println!("NEW BEST: {}", best_remaining);
            dbg!(&next_node.state);
        }


        if next_node.state.eq(&GOAL) { // .eq() required instead of == due to lazy_static GOAL
            println!("Solution found after {} nodes", visited.len());
            return next_node.current_cost;
        } else {       
            for discovered in find_next_states(&next_node) {
                if !visited.contains(&discovered.state) {
                    upsert_node(&mut known, &discovered);
                }
            }
        }
    }
    panic!("No solution found!");
}

fn upsert_node(list: &mut Vec<SearchNode>, new_node: &SearchNode) {
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

fn find_node_idx(list: &mut [SearchNode], target: &SearchNode) -> Option<usize> {
    list.iter_mut()
        .position(|node| search_nodes_equal(node, target))
}

type Coord = (u32, u32); // (y, x)
type CoordPair = [Coord; 2];
type ConnectionMap = HashMap<Coord, Vec<(Coord, u32)>>;

#[derive (Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum AmphipodType {
    A,
    B,
    C,
    D
}

#[derive (Clone, Eq)]
struct State {
    // Required to be a BTreeMap as HashMap does not implement the Hash trait
    locs: BTreeMap<AmphipodType, CoordPair>
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "[{:?} - {:?} - {:?} - {:?}]", self.locs[&AmphipodType::A], self.locs[&AmphipodType::B], self.locs[&AmphipodType::C], self.locs[&AmphipodType::D])
        // write!(f, "[{:?} - {:?} - {:?}]", self.locs[&AmphipodType::A], self.locs[&AmphipodType::B], self.locs[&AmphipodType::D])
    }
}

// Custom Hash needed to ensure states match in the HashMap even if both Amphipods
// of the same type are swapped
impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        each_amphipod_type().for_each( |amp_type| {
            let mut sorted_locs = self.locs[&amp_type];
            sorted_locs.sort();
            sorted_locs.hash(state);
        });
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        each_amphipod_type().all(|amp_type|
            self.locs.contains_key(&amp_type) &&
            other.locs.contains_key(&amp_type) && 
            coord_pairs_equal(&self.locs[&amp_type], &other.locs[&amp_type])
        )
    }
}

#[derive (Clone)]
struct SearchNode {
    state: State,
    current_cost: u32,
    total_cost_estimate: u32
}

fn coords_equal(left: &Coord, right: &Coord) -> bool {
    left.0 == right.0 && left.1 == right.1
}

fn coord_pairs_equal(left: &CoordPair, right: &CoordPair) -> bool {
    (coords_equal(&left[0], &right[0]) && coords_equal(&left[1], &right[1])) ||
    (coords_equal(&left[0], &right[1]) && coords_equal(&left[1], &right[0]))
}

// Nodes in the graph search are considered the same if they have matching states, ignoring costs
fn search_nodes_equal(left: &SearchNode, right: &SearchNode) -> bool {
    left.state == right.state
}

fn each_amphipod_type() -> impl Iterator<Item=AmphipodType> {
    IntoIterator::into_iter([
        AmphipodType::A,
        AmphipodType::B,
        AmphipodType::C,
        AmphipodType::D
    ])
}

fn each_amphipod (state: &State) -> impl Iterator<Item=(AmphipodType, Coord, Coord)>  + '_ {
    let state_clone = state.clone();
    each_amphipod_type().map(
        move |amphipod_type| (amphipod_type, state_clone.locs[&amphipod_type])
    )
    // Check both Amphipods of the type
    .flat_map(|(amphipod_type, amphipod)| {
        vec!(
            (amphipod_type, amphipod[0], amphipod[1]),
            (amphipod_type, amphipod[1], amphipod[0])
        )
    })
}

fn find_next_states(current: &SearchNode) -> Vec<SearchNode> {
    // Any of the Amphipods can attempt to move a space
    each_amphipod(&current.state)
    // Check each possible destination
    .flat_map(|(amphipod_type, amphipod, other_amphipod_of_type)| {
        let amphipod_type_copy = amphipod_type;
        let amphipod_copy = amphipod;
        CONNECTIONS.get(&amphipod).unwrap().iter()
            // Can't move if another amphipod is between the current location and destination
            .filter(move |(new_loc, _)| unblocked_by_other_amphipods(current, amphipod_copy, new_loc))
            // Create new search node with state for moved amphipod
            .map(move |&(new_loc, distance)| {
                let new_state = state_with_moved_location(new_loc, other_amphipod_of_type, &current.state, amphipod_type_copy);
                search_node_for_state(distance, amphipod_type, new_state, current.current_cost)
            })
    })
    .collect()
}

fn search_node_for_state(distance: u32, amphipod_type: AmphipodType, new_state: State, previous_cost: u32) -> SearchNode {
    let additional_cost = distance * MOVEMENT_COSTS[&amphipod_type];
    let remaining_cost = estimate_remaining_cost(&new_state);
    SearchNode {
        state: new_state,
        current_cost: previous_cost + additional_cost,
        total_cost_estimate: previous_cost + additional_cost + remaining_cost
    }
}

fn state_with_moved_location(new_loc: (u32, u32), other_amphipod_of_type: (u32, u32), old_state: &State, amphipod_type_copy: AmphipodType) -> State {
    let mut new_amphipod_locations = [new_loc, other_amphipod_of_type];
    new_amphipod_locations.sort();
    let mut new_state = old_state.clone();
    new_state.locs.insert(amphipod_type_copy, new_amphipod_locations);
    new_state
}

fn unblocked_by_other_amphipods(current: &SearchNode, amphipod_copy: (u32, u32), new_loc: &(u32, u32)) -> bool {
    each_amphipod(&current.state).all(|(_, other_amphipod, _)|
        other_amphipod == amphipod_copy ||
        !blocks_path(&amphipod_copy, new_loc, &other_amphipod)
    )
}

fn estimate_remaining_cost(state: &State) -> u32 {
    // Optimistic estimate of the movement cost required to get both amphipods of each colour to
    // their goals, ignoring all obstacles
    each_amphipod_type().map(|amphipod_type|
        (state.locs[&amphipod_type], GOAL.locs[&amphipod_type], MOVEMENT_COSTS[&amphipod_type])
    )
    .map(|(current_locs, goal_locs, movement_cost)|
        // Compare the two alternatives for matching amphipods to goal spaces
        std::cmp::min(
            walk_distance(&current_locs[0], &goal_locs[0]) + walk_distance(&current_locs[1], &goal_locs[1]),
            walk_distance(&current_locs[0], &goal_locs[1]) + walk_distance(&current_locs[1], &goal_locs[0])
        ) * movement_cost
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
        (obstacle.0 == 0) && ((lower..=higher).contains(&obstacle.1))
    }
}

// Map is hard-coded and unchanging and so we can pre-calculate our
// connectivity map, initial and goal states
// #############
// #...........#
// ###B#D#C#A###
//   #C#D#B#A#
//   #########

lazy_static! {
    static ref INTERSECTIONS: HashSet<Coord> = HashSet::from([
        (0, 2), (0, 4), (0, 6), (0 ,8)
    ]);

    static ref HALLWAY_STOPS: HashSet<Coord> = HashSet::from([
        (0, 0), (0, 1), (0, 3), (0 ,5), (0, 7), (0, 9), (0, 10)
    ]);

    static ref ROOMS: HashMap<Coord, AmphipodType> = HashMap::from([
        ((1, 2), AmphipodType::A),
        ((2, 2), AmphipodType::A),
        ((1, 4), AmphipodType::B),
        ((2, 4), AmphipodType::B),
        ((1, 6), AmphipodType::C),
        ((2, 6), AmphipodType::C),
        ((1, 8), AmphipodType::D),
        ((2, 8), AmphipodType::D)
    ]);

    // Precalculate connectivity and costs for moving from a room into the hallway and vice
    // versa
    static ref CONNECTIONS: ConnectionMap = 
        iproduct!(
            ROOMS.iter().map(|(coord, _)| coord),
            HALLWAY_STOPS.iter()
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
        });

    static ref INITIAL: State = State {
        locs: BTreeMap::from([
            (AmphipodType::A, [(1, 8), (2, 8)]),
            (AmphipodType::B, [(1, 2), (2, 6)]),
            (AmphipodType::C, [(1, 6), (2, 2)]),
            (AmphipodType::D, [(1, 4), (2, 4)])
        ])
    };

    static ref GOAL: State = State {
        locs: BTreeMap::from([
            (AmphipodType::A, [(1, 2), (2, 2)]),
            (AmphipodType::B, [(1, 4), (2, 4)]),
            (AmphipodType::C, [(1, 6), (2, 6)]),
            (AmphipodType::D, [(1, 8), (2, 8)])
        ])
    };

    static ref MOVEMENT_COSTS: HashMap::<AmphipodType, u32> = HashMap::from([
        (AmphipodType::A, 1),
        (AmphipodType::B, 10),
        (AmphipodType::C, 100),
        (AmphipodType::D, 1000)
    ]);
}