use std::{fmt, ptr::eq};
use itertools::iproduct;

use crate::utils::read_string_lines;

pub fn solution18 () {
    let input: Vec<SnailfishNumber> = read_string_lines("src/data/solution18.txt").iter()
        .map(String::as_str)
        .map(parse_snailfish_number)
        .collect();
    println!("{}", solution18a(&input));
    println!("{}", solution18b(&input));
}

fn solution18a(input: &[SnailfishNumber]) -> u32 {
    // Just add all of the Snailfish numbers together and determine magnitude
    input.iter()
        .cloned()
        .reduce(add_numbers)
        .expect("Input data is empty of valid Snailfish numbers")
        .magnitude()
}

fn solution18b(input: &[SnailfishNumber]) -> u32 {
    // Cartesian product to find each pair of numbers
    iproduct!(input, input)
        // Numbers must be different from each other
        .filter(|(left, right)| !eq(*left, *right))    
        // Find magnitude of their sum
        .map(|(left, right)|
            add_numbers(left.clone(), right.clone()).magnitude()
        )
        // We are interested in only the biggest result
        .max()
        .expect("Input data is empty of valid Snailfish numbers")
}

fn add_numbers(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    let mut combined = SnailfishNumber(
        Box::new(
            Pair {
                left: Node::Pair(left.0),
                right: Node::Pair(right.0)
            }
        )
    );

    // Check for explodes and splits until none are required
    while combined.0.try_explode_children(1).exploded || combined.0.try_split_children() { }

    combined
}

const SPLIT_LIMIT: u32 = 10;
const OUTER_PAIR_LIMIT: u32 = 4;

trait Magnitude { 
    fn magnitude(&self) -> u32;
}

#[derive(Clone)]
struct SnailfishNumber(Box<Pair>);

impl fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number {:?}", self.0)
    }
}

impl Magnitude for SnailfishNumber {
    fn magnitude(&self) -> u32 {
        self.0.magnitude()
    }
}

#[derive(Clone)]
struct Pair {
    left: Node,
    right: Node,
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?} , {:?}]", self.left, self.right)
    }
}

impl Magnitude for Pair {
    fn magnitude(&self) -> u32 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }
}

#[derive(Clone)]
enum Node {
    Pair(Box<Pair>),
    Value(u32)
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Node::Value(value) => write!(f, "{}", value),
            Node::Pair(pair) => pair.fmt(f)
        }
    }
}

impl Magnitude for Node {
    fn magnitude(&self) -> u32 {
        match self {
            Node::Pair(pair) => pair.magnitude(), 
            Node::Value(value) => *value
        }
    }
}

struct ExplodeResult {
    exploded: bool,
    carry_value: Option<ExplodeCarryValue>,
}

struct ExplodeCarryValue {
    pair_half: PairHalf,
    value: u32
}

#[derive(PartialEq)]
enum PairHalf {Left, Right}

impl Pair {
    fn create_carry_values(&self, split_half: PairHalf) -> (ExplodeCarryValue, ExplodeCarryValue) {
        let left_carry_value = ExplodeCarryValue{
            pair_half: PairHalf::Left,
            value: self.left.force_as_value()
        };
        let right_carry_value = ExplodeCarryValue{
            pair_half: PairHalf::Right,
            value: self.right.force_as_value()
        };

        if split_half == PairHalf::Left {
            (left_carry_value, right_carry_value)
        } else {
            (right_carry_value, left_carry_value)
        }
    }

    fn try_explode_children(&mut self, outer_pairs: u32) -> ExplodeResult {
        // At outer pair limit, any children that are pairs are ready to explode
        // NOTE: There is an assumption that an exploding pair will have values as both children, as regular
        // exploding after each add should not result in a pair reaching the depth limit while having further
        // pairs beneath them
        if outer_pairs >= OUTER_PAIR_LIMIT {
            // Check left before right
            for check_half in vec!(PairHalf::Left, PairHalf::Right) {
                let (child, other_child) = if check_half == PairHalf::Left {
                    (&mut self.left, &mut self.right)
                } else {
                    (&mut self.right, &mut self.left)
                };

                if let Node::Pair(pair) = child {
                    // The exploded child is broken into two carried values and is set to zero
                    let (propagate_carry, other_child_accepts) = pair.create_carry_values(check_half);
                    *child = Node::Value(0);
                    // One of the split values is accepted by the other child
                    other_child.accept_carry_value(&other_child_accepts);
                    // The other split value must propagate up the tree and then down again
                    return ExplodeResult {
                        exploded: true,
                        carry_value: Some(propagate_carry)
                    };
                }
            }
            
        } else {
            // Otherwise a recursive search through child pairs. Propagate upwards any reports of
            // explosions. An explosion may also come with a left- or right- fragment that needs to
            // be shifted left or right along the tree. In practical terms this involves moving the
            // fragment up the tree and then down again.
            // Check left before right
            for check_half in vec!(PairHalf::Left, PairHalf::Right) {
                let (child, other_child) = if check_half == PairHalf::Left {
                    (&mut self.left, &mut self.right)
                } else {
                    (&mut self.right, &mut self.left)
                };
            
                if let Node::Pair(pair) = child {
                    let mut explode_attempt = pair.try_explode_children(outer_pairs+1);
                    if explode_attempt.exploded {
                        if let Some(carry_value) = &explode_attempt.carry_value {
                            if carry_value.pair_half != check_half {
                                // Safe to unwrap as we already matched against Some above
                                other_child.accept_carry_value(&explode_attempt.carry_value.unwrap());
                                explode_attempt.carry_value = None;
                            }
                        }
                        return explode_attempt;
                    }
                }
            }
        }
        // If reached, no explodes are required
        ExplodeResult{
            exploded: false,
            carry_value: None
        }                
    }

    fn try_split_children(&mut self) -> bool {
        for child in [&mut self.left, &mut self.right] {
            if match child {
                Node::Pair(pair) => pair.try_split_children(),
                Node::Value(value) if (*value >= SPLIT_LIMIT) => {
                    child.split();
                    true
                },
                _ => false
            } {
                return true;
            }
        }
        false // No splits required
    }
}

impl Node {
    fn force_as_value(&self) -> u32 {
        if let Self::Value(value) = self {*value} else { dbg!(&self); unreachable!("Forcing non-value node to value!")}
    }

    fn split(&mut self) {
        match self {
            Self::Value(value) => {
                let left_val = *value / 2; // Left half rounds down (integer division)
                *self = Self::Pair(
                    Box::new(
                        Pair {
                            left: Self::Value(left_val),
                            right: Self::Value(*value - left_val)
                        }
                    )
                );

            },
            _ => unimplemented!("Only value nodes are splittable!")
        }
    }

    fn accept_carry_value(&mut self, carry_value: &ExplodeCarryValue) {
        match (self, carry_value) {
            (Node::Value(my_value), ExplodeCarryValue{value: carried, ..}) => {
                *my_value += carried;
            },
            (Node::Pair(pair), ExplodeCarryValue{pair_half: PairHalf::Right, ..}) => {
                pair.left.accept_carry_value(carry_value)
            },
            (Node::Pair(pair), ExplodeCarryValue{pair_half: PairHalf::Left, ..}) => {
                pair.right.accept_carry_value(carry_value)
            }           
        };
    }
}

fn parse_snailfish_number(num_ser: &str) -> SnailfishNumber {
    SnailfishNumber(
        Box::new(
            parse_pair(&num_ser[1..num_ser.len()-1])
        )
    )    
}

fn parse_node(node_ser: &str) -> Node {
    if !node_ser.starts_with('[') {
        Node::Value(node_ser.parse::<u32>().expect("Invalid Snailfish number"))
    } else {
        Node::Pair(
            Box::new(parse_pair(&node_ser[1..node_ser.len()-1]))
        )
    }
}

fn parse_pair(pair_ser: &str) -> Pair {
    let comma_pos = find_comma(pair_ser);
    Pair{
        left: parse_node(&pair_ser[..comma_pos]),
        right: parse_node(&pair_ser[(comma_pos+1)..])
    }
}

fn find_comma(pair_ser: &str) -> usize {
    let mut stack_count = 0;
    for (idx, char) in pair_ser.chars().enumerate() {
        match char {
            ',' if stack_count == 0 => return idx,
            '[' => stack_count += 1,
            ']' if stack_count == 0 => panic!("Unexpected pair finish!"),
            ']' => stack_count -= 1,
            _ => ()
        }
    }
    unreachable!("Failed to find comma in pair!");
}