use crate::utils::read_string_lines;

pub fn solution18 () {
    let input: Vec<SnailfishNumber> = read_string_lines("src/data/solution18.txt")[3..4].iter()
        .map(String::as_str)
        .map(parse_snailfish_number)
        .collect();
    println!("{}", solution18a(&input));
}

fn solution18a(input: &[SnailfishNumber]) -> u32 {
    input.iter()
    .cloned()
    .reduce(add_numbers)
    .expect("Input data is empty of valid Snailfish numbers")
    .magnitude()
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

    // Checks for explodes and splits until none are required
    while combined.0.try_explode_children(0).exploded || combined.0.try_split_children() { }

    combined
}

const SPLIT_LIMIT: u32 = 10;
const OUTER_PAIR_LIMIT: u32 = 4;

#[derive(Clone)]
struct SnailfishNumber(Box<Pair>);

#[derive(Clone)]
struct Pair {
    left: Node,
    right: Node,
}

#[derive(Clone)]
enum Node {
    Pair(Box<Pair>),
    Value(u32)
}

struct ExplodeResult {
    exploded: bool,
    part: ExplodePart,
}

enum ExplodePart {
    None,
    Left(u32),
    Right(u32)
}

impl Pair {
    fn try_explode_children(&mut self, outer_pairs: u32) -> ExplodeResult {
        // At outer pair limit, any children that are pairs are ready to explode
        if outer_pairs >= OUTER_PAIR_LIMIT {
            // Important to check left then right separately, due to slight differences in propagation
            // NOTE: There is an assumption that an exploding pair will have values as both children, as regular
            // exploding after each add should not result in a pair reaching the depth limit while having further
            // pairs beneath them
            if let Node::Pair(pair) = &mut self.left {
                self.left = Node::Value(0);
                self.right.accept_explode_part(ExplodePart::Right(pair.right.force_as_value()));
                return ExplodeResult {
                    exploded: true,
                    part: ExplodePart::Left(self.left.force_as_value())
                };
            } else if let Node::Pair(pair) = &mut self.right {
                self.right = Node::Value(0);
                self.left.accept_explode_part(ExplodePart::Left(pair.left.force_as_value()));
                return ExplodeResult {
                    exploded: true,
                    part: ExplodePart::Right(self.right.force_as_value())
                };
            }
        } else {
            // TODO: COMMENT
            for (child, is_left) in [(&mut self.left, true), (&mut self.right, false)] {
                if let Node::Pair(pair) = child {
                    let mut explode_attempt = pair.try_explode_children(outer_pairs+1);
                    if  explode_attempt.exploded {
                        if is_left {
                            if let ExplodePart::Right(value) = explode_attempt.part {
                                explode_attempt.part = ExplodePart::None;
                                self.right.accept_explode_part(value);
                            }
                        } else {
                            if let ExplodePart::Left(value) = explode_attempt.part {
                                explode_attempt.part = ExplodePart::None;
                                self.left.accept_explode_part(value);
                            }
                        }
                        return explode_attempt;
                    }
                }
            }
        }
        ExplodeResult{
            exploded: false,
            part: ExplodePart::None
        } // No explodes required                
    }

    fn try_split_children(&mut self) -> bool{
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
        if let Self::Value(value) = self {*value} else {unreachable!("Forcing non-value node to value!")}
    }

    fn split(&mut self) {
        match self {
            Self::Value(value) => {
                let left_val = *value / 2;
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
}

trait Magnitude { 
    fn magnitude(&self) -> u32;
}

impl Magnitude for SnailfishNumber {
    fn magnitude(&self) -> u32 {
        self.0.magnitude()
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

impl Magnitude for Pair {
    fn magnitude(&self) -> u32 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
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