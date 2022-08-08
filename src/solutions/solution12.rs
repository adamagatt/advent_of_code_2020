use crate::utils::read_string_pairs;

use std::collections::{HashSet, HashMap};

struct Cave {
    name: String,
    revisitable: bool,
    connections: HashSet<String>
}
type CaveMap = HashMap<String, Cave>;

pub fn solution12 () {
    let cave_map = build_cave_map(&read_string_pairs("src/data/solution12.txt", '-'));
    println!("{}", solution12a(&cave_map));
    println!("{}", solution12b(&cave_map));
}

fn solution12a(cave_map: &CaveMap) -> i32 {
    let visited_caves = HashSet::<String>::new();
    explore_cave(&cave_map["start"], cave_map, &visited_caves, false)
}

fn solution12b(cave_map: &CaveMap) -> i32 {
    let visited_caves = HashSet::<String>::new();
    explore_cave(&cave_map["start"], cave_map, &visited_caves, true)
}

fn build_cave_map(connections: &Vec<(String, String)>) -> CaveMap {
    let mut cave_map = CaveMap::new();
    for (cave1, cave2) in connections {
        // Connecting in both directions
        for (&src_cave, &dest_cave) in [(&cave1, &cave2), (&cave2, &cave1)] {
            let src_cave_name = String::from(src_cave);
            let dest_cave_name = String::from(dest_cave);
            let cave_map_entry = cave_map.entry(src_cave_name.clone())
                // If the source cave isn't in our map yet then create its entry
                .or_insert(Cave {
                    name: src_cave_name.clone(),
                    revisitable: (src_cave_name == src_cave_name.to_uppercase()),
                    connections: HashSet::<String>::new()
                });
            // Add the connection from the source cave to the destination cave
            cave_map_entry.connections.insert(dest_cave_name);
        }
    }
    cave_map
}

fn explore_cave(cave: &Cave, cave_map: &CaveMap, visited: &HashSet<String>, can_revisit: bool) -> i32 {
    // Need a new visited set for searching the child nodes as we can't add to the
    // same reference set
    let mut new_visited = visited.clone();
    new_visited.insert(cave.name.clone());

    match cave.name.as_str() {
        // If we get to "end" we can terminate and return the success of this path
        "end" => 1,
        // Otherwise we examine all connections out of this cave
        _ => cave.connections.iter()
            .map(|dest| &cave_map[dest])
            .filter_map(|dest_cave| {
                let dest_cave_name = &dest_cave.name;
                // Visiting a revisitable cave or any cave for the first time preserves
                // our revisiting status
                if dest_cave.revisitable || !visited.contains(dest_cave_name) {
                    Some((dest_cave, can_revisit))
                } else if dest_cave_name != "start" // If there is a non-start cave...
                    && visited.contains(dest_cave_name) // ... that we had already visited ...
                    && can_revisit { // ... but we still have our single revisit allowance
                    // Then we can visit the cave by spending our revisit
                    Some((dest_cave, false))
                } else {
                    // An un-revisitable cave that we have already visited when we have used
                    // up our single revisit means that we cannot explore here. Returning None
                    // will cause the filter_map operation to filter this branch out.
                    None
                }
            })
            .map(|(dest_cave, new_can_revisit)| explore_cave(dest_cave, cave_map, &new_visited, new_can_revisit))
            .sum() // We want the sum of descendents that eventually get to "end" (return 1)
    }
}