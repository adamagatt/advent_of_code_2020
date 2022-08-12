use crate::utils::read_string_lines;

pub fn solution18 () {

    let input: Vec<SnailfishNumber> = read_string_lines("src/data/solution18.txt").iter()
        .map(String::as_str)
        .map(parse_snailfish_number)
        .collect();
    println!("{}", solution18a(&input));
}

struct SnailfishNumber(Box<Pair>);

struct Pair {
    left: Node,
    right: Node,
}

enum Node {
    Pair(Box<Pair>),
    Value(i32)
}

fn solution18a(input: &[SnailfishNumber]) -> i32 {
    5
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
    5
}