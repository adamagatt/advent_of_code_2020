use crate::utils::read_string_lines;

pub fn solution18 () {
    let input: Vec<SnailfishNumber> = read_string_lines("src/data/solution18.txt")[3..4].iter()
        .map(String::as_str)
        .map(parse_snailfish_number)
        .collect();
    println!("{}", solution18a(&input));
}

fn solution18a(input: &[SnailfishNumber]) -> i32 {
    input.iter()
    .cloned()
    .reduce(add_numbers)
    .expect("Input data is empty of valid Snailfish numbers")
    .magnitude()
}

fn add_numbers(left: SnailfishNumber, right: SnailfishNumber) -> SnailfishNumber {
    let combined = SnailfishNumber(
        Box::new(
            Pair {
                left: Node::Pair(left.0),
                right: Node::Pair(right.0)
            }
        )
    );

    combined
}

#[derive(Clone)]
struct SnailfishNumber(Box<Pair>);

impl Magnitude for SnailfishNumber {
    fn magnitude(&self) -> i32 {
        self.0.magnitude()
    }
}

#[derive(Clone)]
struct Pair {
    left: Node,
    right: Node,
}

#[derive(Clone)]
enum Node {
    Pair(Box<Pair>),
    Value(i32)
}

trait Magnitude { 
    fn magnitude(&self) -> i32;
}

impl Magnitude for Node {
    fn magnitude(&self) -> i32 {
        match self {
            Node::Pair(pair) => pair.magnitude(), 
            Node::Value(value) => value.magnitude()
        }
    }
}

impl Magnitude for i32 {
    fn magnitude(&self) -> i32 {
        *self
    }
}

impl Magnitude for Pair {
    fn magnitude(&self) -> i32 {
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
        Node::Value(node_ser.parse::<i32>().expect("Invalid Snailfish number"))
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
            ']' => stack_count -=1,
            _ => ()
        }
    }
    unreachable!("Failed to find comma in pair!");
}