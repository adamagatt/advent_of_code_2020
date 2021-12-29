use crate::utils::read_string_lines;

pub fn solution16 () {
    let code_str = read_string_lines("src/data/solution16.txt").remove(0);
    let bit_string = parse_bytes(&code_str);
    // Can parse the packet now to provide to sub-problems
    let root_packet = parse_packet(&bit_string);
    println!("{}", solution16a(&root_packet));
    println!("{}", solution16b(&root_packet));
}

fn solution16a(root_packet: &Packet) -> u32 {
    root_packet.version_sum()
}

fn solution16b(root_packet: &Packet) -> u32 {
    root_packet.eval()
}

// Convert from hex to bitstring (in this case, vector of bools)
fn parse_bytes(code_str: &String) -> Vec<bool> {
    hex::decode(code_str).expect("Supplied code isn't valid hex").iter()
        // Flatmap to convert each byte in stream to concatenated sub-stream of 8 bits
        .flat_map(|byte| [
            byte & 128 == 128, byte & 64 == 64, byte & 32 == 32, byte & 16 == 16,
            byte &   8 ==   8, byte &  4 ==  4, byte &  2 ==  2, byte &  1 ==  1
        ])
        .collect::<Vec<bool>>()
}

fn print_bits(bit_string: &[bool], length: usize) -> () {
    println!("{}", bit_string.iter().take(length).map(|&x| if x {'1'} else {'0'}).collect::<String>());
}

// Parse a specified number of bits into a numeric value 
fn read_num(bit_string: &[bool], length: usize) -> u32 {
    let mut output = 0u32;
    
    for idx in 0..length {
        if bit_string[idx] {
            output += 1 << (length - idx - 1);
        }
    }
    output
}

fn parse_packet(bit_string: &[bool]) -> Packet {
    let version = read_num(&bit_string, 3) as u8;
    // Read operator to determine the type of patcket this is
    let (data, data_length) = match read_num(&bit_string[3..], 3) {
        4 => read_literal(&bit_string[6..]),
        _ => parse_subpackets(&bit_string[6..])
    };

    Packet{
        version, data,
        bit_length: data_length + 6
    }
}

// Use 5-bit batch parsing method to read literal values
fn read_literal(bit_string: &[bool]) -> (Data, usize) {
    let mut groups = Vec::<u32>::new();
    let mut cur_index = 0;

    loop {
        groups.push(read_num(&bit_string[cur_index+1..], 4));
        // Only proceed to next batch if leading bit is 1
        if !bit_string[cur_index] {
            break;
        }
        cur_index += 5;
    }

    let data = Data::Literal(
        groups.iter()
            .copied()
            .reduce(|acc, group| (acc << 4 + group))
            .expect("Literal data had no groups")
            .clone()
    );

    (data, groups.len() * 5)
    
}

fn parse_subpackets(bit_string: &[bool]) -> (Data, usize) {
    let length = if bit_string[0] {
        // First bit ON indicates number of subpackets
        Length::Subpackets(read_num(&bit_string[1..], 11) as usize)
    } else {
        // First bit OFF indicates total bit length
        Length::TotalBits(read_num(&bit_string[1..], 15) as usize)
    };

    let mut subpackets = Vec::<Packet>::new();

    // Track both subpackets and bits read to allow stopping on either strategy
    let mut total_bit_length: usize = 0;
    let mut subpacket_count: usize = 0;
    let header_bits = 1 + if let Length::TotalBits(_) = length {15} else {11};

    loop {
        let subpacket = parse_packet(&bit_string[(header_bits+total_bit_length)..]);

        total_bit_length += subpacket.bit_length;
        subpacket_count += 1;

        subpackets.push(subpacket);

        // The condition to stop reading depends on the length strategy
        if match length {
            Length::TotalBits(bits) => total_bit_length == bits,
            Length::Subpackets(packets) => subpacket_count == packets
        } {
            break;
        }
    }

    let data = Data::Operator(
        OperatorPacket { length, subpackets }
    );

    (data, header_bits + total_bit_length)
}

#[derive(PartialEq)]
enum Length {
    // Subpacket contents sum to a particular total of bits
    TotalBits(usize),
    // Subpacket contents have a particular count
    Subpackets(usize)
}

enum Data {
    Literal(u32),
    Operator(OperatorPacket)
}

struct Packet {
    version: u8,
    bit_length: usize,
    data: Data
}

impl Packet {
    fn version_sum(&self) -> u32 {
        (self.version as u32)
        + match &self.data {
            Data::Literal(_) => 0,
            Data::Operator(packet) => packet.subpackets.iter()
                .map(|subpacket| subpacket.version_sum())
                .sum()
        }
    }

    fn eval(&self) -> u32 {
        5
    }
}

struct OperatorPacket {
    length: Length,
    subpackets: Vec<Packet>
}