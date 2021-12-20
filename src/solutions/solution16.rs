use crate::utils::read_string_lines;

pub fn solution16 () {
    let code_str = read_string_lines("src/data/solution16.txt").remove(0);
    let bit_string = parse_bytes(&code_str);
    println!("{}", solution16a(&bit_string));
    println!("{}", solution16b(&bit_string));
}

fn solution16a(bit_string: &Vec<bool>) -> u32 {
    let packet = decode_packet(bit_string);
    5
}

fn solution16b(bit_string: &Vec<bool>) -> u32 {
    5
}

fn parse_bytes(code_str: &String) -> Vec<bool> {
    hex::decode(code_str).expect("Supplied code isn't valid hex").iter()
        .flat_map(|byte| [
            byte & 128 == 128, byte & 64 == 64, byte & 32 == 32, byte & 16 == 16,
            byte &   8 ==   8, byte &  4 ==  4, byte &  2 ==  2, byte &  1 ==  1
        ])
        .collect::<Vec<bool>>()
}

fn read_num(bit_string: &[bool], length: usize) -> u32 {
    let mut output = 0u32;
    for idx in 0..length {
        if bit_string[idx] {
            output += 1 << (length - idx - 1);
        }
    }
    output
}

fn decode_packet(bit_string: &[bool]) -> Packet {
    let version = read_num(&bit_string, 3) as u8;
    let data = match read_num(&bit_string[3..], 3) {
        4 => read_literal(&bit_string[6..]),
        _ => parse_subpackets(&bit_string[6..])
    };

    Packet{ version, data }
}

fn read_literal(bit_string: &[bool]) -> Data {
    let mut groups = Vec::<u32>::new();
    let mut curIndex = 0;

    loop {
        groups.push(read_num(&bit_string[curIndex+1..], 4));
        if !bit_string[curIndex] {
            break;
        }
        curIndex += 4;
    }

    return Data::Literal(
        groups.iter()
            .copied()
            .reduce(|acc, group| (acc << 4 + group))
            .expect("Literal data had no groups")
            .clone()
    )
}

fn parse_subpackets(bit_string: &[bool]) -> Data {
    let (length_id, length_value) = if read_num(&bit_string, 1) == 0 {
        (LengthId::TotalLength, read_num(&bit_string[1..], 15))
    } else {
        (LengthId::SubpacketCount, read_num(&bit_string[1..], 11))
    };
    
    let subpackets = Vec::<Packet>::new();

    Data::Operator(
        OperatorPacket { length_id, length_value, subpackets }
    )
}

enum LengthId {
    TotalLength = 0,
    SubpacketCount = 1
}

enum Data {
    Literal(u32),
    Operator(OperatorPacket)
}

struct Packet {
    version: u8,
    data: Data
}

struct OperatorPacket {
    length_id: LengthId,
    length_value: u32,
    subpackets: Vec<Packet>
}