use lazy_static::lazy_static;
use std::collections::HashMap;

pub fn solution23 () {
    println!("{}", solution23a());
//    println!("{}", solution23b());
}

fn solution23a() -> usize {
    5
}

type Coord = (usize, usize); // (y, x)
type CoordPair = [Coord; 2];

#[derive (Clone, Eq, PartialEq, Hash)]
enum AmphipodType {A, B, C, D}

#[derive (Clone)]
struct State {
    locations: HashMap<AmphipodType, CoordPair>,
    cost: u32
}

fn coords_equal(left: &Coord, right: &Coord) -> bool {
    left.0 == right.0 && left.1 == right.1
}

fn coord_pairs_equal(left: &CoordPair, right: &CoordPair) -> bool {
    (coords_equal(&left[0], &right[0]) && coords_equal(&left[1], &right[1])) ||
    (coords_equal(&left[0], &right[1]) && coords_equal(&left[1], &right[0]))
}

fn states_equal(left: &State, right: &State) -> bool {
    left.locations.keys().eq(right.locations.keys()) &&
    left.locations.keys()
        .all(|key| {
            let left_coord_pair = left.locations.get(key).unwrap();
            let right_coord_pair = right.locations.get(key).unwrap();
            coord_pairs_equal(left_coord_pair, right_coord_pair)
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

    static ref GOAL: State = State {
        locations: HashMap::from([
            (AmphipodType::A, [(1, 2), (2, 2)]),
            (AmphipodType::B, [(1, 4), (2, 4)]),
            (AmphipodType::C, [(1, 6), (2, 6)]),
            (AmphipodType::D, [(1, 8), (2, 8)])
        ]),
        cost: 0 // Cost isn't relevant for determining if goal is reached or not
    };

    static ref INITIAL: State = State {
        locations: HashMap::from([
            (AmphipodType::A, [(1, 8), (2, 8)]),
            (AmphipodType::B, [(1, 2), (2, 6)]),
            (AmphipodType::C, [(1, 6), (2, 2)]),
            (AmphipodType::D, [(1, 4), (2, 4)])
        ]),
        cost: 0
    };
}