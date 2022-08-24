use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, BTreeMap};

pub fn solution23 () {
    println!("{}", solution23a());
//    println!("{}", solution23b());
}

fn solution23a() -> u32 {
    // States adjacent to those we have visited
    let mut known = Vec::<SearchNode>::new();
    // States we have visited and won't need to consider again
    let mut visited: HashSet<State> = HashSet::from([INITIAL.clone()]);

    while !known.is_empty() {
        let next_node = known.remove(0);
        visited.insert(next_node.state.clone());

        if states_equal(&next_node.state, &GOAL) {
            return next_node.current_cost;
        } else {
            for discovered in find_next_states(&next_node) {
                if !visited.contains(&discovered.state) {
                    insert_node(&mut known, &discovered);
                }
            }
        }
    }
    panic!("No solution found!");
}

fn insert_node(list: &mut Vec<SearchNode>, node: &SearchNode) {
    if let Some(found) = get_node(list, node) {
        // If the node is already in the list then lower its cost if necessary
        if node.current_cost < found.current_cost {
            found.current_cost = node.current_cost;
            found.total_cost_estimate = node.total_cost_estimate;
        }
    } else {
        // Add to list if not previously present
        list.push(node.clone());
    }

    // Re-sort the list, since we may have added a node or revised a cost. Sorting is done
    // by "total" cost, a combination of the known cost to get there and the guess of
    // the remaining distance
    list.sort_by_key(|node| node.total_cost_estimate);
}

fn get_node<'a, 'b>(list: &'a mut [SearchNode], target: &'b SearchNode) -> Option<&'a mut SearchNode> {
    list.iter_mut()
        .find(|node| states_equal(&node.state, &target.state))
}

type Coord = (u32, u32); // (y, x)
type CoordPair = [Coord; 2];

#[derive (Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum AmphipodType {A, B, C, D}

#[derive (Clone, PartialEq, Eq, Hash)]
struct State {
    // Required to be a BTreeMap as HashMap does not implement the Hash trait
    locs: BTreeMap<AmphipodType, CoordPair>
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

fn states_equal(left: &State, right: &State) -> bool {
    each_amphipod_type().all(|amphipodType|
        left.locs.contains_key(amphipodType) &&
        right.locs.contains_key(amphipodType) &&
        coord_pairs_equal(&left.locs[&amphipodType], &right.locs[&amphipodType])
    )
}

// Nodes in the graph search are considered the same if they have matching states, ignoring costs
fn search_nodes_equal(left: &SearchNode, right: &SearchNode) -> bool {
    states_equal(&left.state, &right.state)
}

fn each_amphipod_type() -> impl Iterator<Item=&'static AmphipodType> {
    [AmphipodType::A, AmphipodType::B, AmphipodType::C, AmphipodType::D].iter()
}

fn find_next_states(current: &SearchNode) -> Vec<SearchNode> {
    let new_states = vec!(current.state.clone());

    // Any of the Amphipod types can attempt to move a space
    each_amphipod_type().map(
        |amphipod_type| (current.state.locs[amphipod_type], MOVEMENT_COSTS[amphipod_type])
    )
      // Check both Amphipods of the type
    .flat_map(|(amphipod, movement_cost)| {
        vec!(
            ((amphipod[0], amphipod[1]), movement_cost),
            ((amphipod[1], amphipod[0]), movement_cost)
        )
    })
    // Check each possible destination
    .flat_map(|((amphipod, other_amphipod), movement_cost)| {
        CONNECTIONS.get(&amphipod).unwrap().iter()
            .map(|new_loc| true)
    });

    new_states.iter()
        .map(|state| {
            let additional_cost = 1;
            let remaining_cost = estimate_remaining_cost(state);
            SearchNode {
                state: state.clone(),
                current_cost: current.current_cost + additional_cost,
                total_cost_estimate: current.current_cost + 1 + remaining_cost
            }
        })
        .collect()
}

fn estimate_remaining_cost(state: &State) -> u32 {
    // Calculated as simple manhattan distance to get both amphipods of each colour to
    // their goal.
    each_amphipod_type().map(|amphipod_type|
        (state.locs[amphipod_type], GOAL.locs[amphipod_type], MOVEMENT_COSTS[amphipod_type])
    )
    .map(|(current_locs, goal_locs, movement_cost)|
        // Compare the two alternatives for matching amphipods to goal spaces
        std::cmp::min(
            manhattan(&current_locs[0], &goal_locs[0]) + manhattan(&current_locs[1], &goal_locs[1]),
            manhattan(&current_locs[0], &goal_locs[1]) + manhattan(&current_locs[1], &goal_locs[0])
        ) * movement_cost
    )
    .sum()
}

fn manhattan(a: &Coord, b: &Coord) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

// Map is hard-coded and unchanging and so we can pre-calculate our
// connectivity map, initial and goal states
// #############
// #...........#
// ###B#D#C#A###
//   #C#D#B#A#
//   #########

lazy_static! {
    static ref CONNECTIONS: HashMap<Coord, Vec<Coord>> = HashMap::from([
        ((0,  0), vec!((0,  1))),
        ((0,  1), vec!((0,  0), (0,  2))),
        ((0,  2), vec!((0,  1), (0,  3), (1,  2))),
        ((0,  3), vec!((0,  2), (0,  4))),
        ((0,  4), vec!((0,  3), (0,  5), (1,  4))),
        ((0,  5), vec!((0,  4), (0,  6))),
        ((0,  6), vec!((0,  5), (0,  7), (1,  6))),
        ((0,  7), vec!((0,  6), (0,  8))),
        ((0,  8), vec!((0,  7), (0,  9), (1,  8))),
        ((0,  9), vec!((0,  8), (0, 10))),
        ((0, 10), vec!((0,  9))),
        ((1,  2), vec!((0,  2), (2,  2))),
        ((2,  2), vec!((1,  2))),
        ((1,  4), vec!((0,  4), (2,  4))),
        ((2,  4), vec!((1,  4))),
        ((1,  6), vec!((0,  6), (2,  6))),
        ((2,  6), vec!((1,  6))),
        ((1,  8), vec!((0,  8), (2,  8))),
        ((2,  8), vec!((1,  8)))
    ]);

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