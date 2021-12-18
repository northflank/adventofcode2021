use std::fs;
use std::iter::Peekable;

type Bit = u8;

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        type_id: u8,
        value: u128,
    },
    Operator {
        version: u8,
        type_id: u8,
        packets: Vec<Packet>,
    },
}

impl Packet {
    fn version(&self) -> u8 {
        match self {
            Packet::Literal { version, .. } => *version,
            Packet::Operator { version, .. } => *version,
        }
    }

    // Task 1
    fn flatten(&self) -> Vec<&Packet> {
        match self {
            Packet::Literal { .. } => vec![self],
            Packet::Operator {
                packets: sub_packets,
                ..
            } => std::iter::once(self)
                .chain(sub_packets.iter().flat_map(|p| p.flatten()))
                .collect(),
        }
    }

    // Task 2
    fn eval(&self) -> u128 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                packets, type_id, ..
            } => match *type_id {
                0 => packets.iter().map(|p| p.eval()).sum(),
                1 => packets.iter().map(|p| p.eval()).product(),
                2 => packets.iter().map(|p| p.eval()).min().unwrap(),
                3 => packets.iter().map(|p| p.eval()).max().unwrap(),
                5 => (packets[0].eval() > packets[1].eval()) as u128,
                6 => (packets[0].eval() < packets[1].eval()) as u128,
                7 => (packets[0].eval() == packets[1].eval()) as u128,
                _ => unreachable!(),
            },
        }
    }
}

type BitIterator = dyn Iterator<Item = Bit>;

struct BitStream {
    iter: Peekable<Box<BitIterator>>,
}

impl BitStream {
    fn from_iter<I: 'static + Iterator<Item = Bit>>(iter: I) -> BitStream {
        let iter: Box<BitIterator> = Box::new(iter);
        BitStream {
            iter: iter.peekable(),
        }
    }

    fn from_vec(data: Vec<Bit>) -> BitStream {
        BitStream::from_iter(data.into_iter())
    }

    fn from_hex_str(data: &str) -> BitStream {
        BitStream::from_vec(
            data.trim()
                .chars()
                .flat_map(|c| {
                    let val = Bit::from_str_radix(&c.to_string(), 16).unwrap();
                    let bits = (0..4).rev().map(move |i| (val >> i) & 1);
                    bits
                })
                .collect(),
        )
    }

    fn from_binary_str(data: &str) -> BitStream {
        BitStream::from_vec(
            data.trim()
                .chars()
                .map(|c| Bit::from_str_radix(&c.to_string(), 2).unwrap())
                .collect(),
        )
    }

    fn read_bits(&mut self, n: usize) -> u128 {
        self.iter
            .by_ref()
            .take(n)
            .fold(0, |a, b| (a << 1) + b as u128)
    }

    fn read_bit(&mut self) -> bool {
        self.read_bits(1) > 0
    }

    fn read_packet(&mut self) -> Packet {
        let version = self.read_bits(3) as u8;
        let type_id = self.read_bits(3) as u8;

        let result = match type_id {
            4 => {
                let value = std::iter::successors(Some(self.read_bits(5)), |a| {
                    if a >> 4 & 1 != 0 {
                        Some(self.read_bits(5))
                    } else {
                        None
                    }
                })
                .fold(0, |a, b| (a << 4) + (b & (0xF)));

                Packet::Literal {
                    version,
                    type_id,
                    value,
                }
            }
            _ => {
                let sub_packets: Vec<Packet> = match self.read_bit() {
                    true => {
                        let sub_count = self.read_bits(11);
                        (0..sub_count).map(|_| self.read_packet()).collect()
                    }
                    false => {
                        let sub_bits = self.read_bits(15);
                        let sub_iter = self
                            .iter
                            .by_ref()
                            .take(sub_bits as usize)
                            .collect::<Vec<_>>()
                            .into_iter();

                        let mut bit_stream = BitStream::from_iter(sub_iter);

                        let mut result = vec![];
                        while bit_stream.iter.peek().is_some() {
                            result.push(bit_stream.read_packet());
                        }

                        result
                    }
                };

                Packet::Operator {
                    version,
                    type_id,
                    packets: sub_packets,
                }
            }
        };

        result
    }
}

fn main() {
    let data = fs::read_to_string("input-day-16.txt").unwrap();

    // let data = "110100101111111000101000";
    // let data = "00111000000000000110111101000101001010010001001000000000";
    // let data = "11101110000000001101010000001100100000100011000001100000";

    let mut stream = BitStream::from_hex_str(&data);

    let packet = stream.read_packet();

    println!("{:?}", packet);
    println!("{:?}", stream.iter.by_ref().map(|a| a).collect::<Vec<_>>());

    let version_sum: u128 = packet.flatten().iter().map(|a| a.version() as u128).sum();

    println!("Task 1: {}", version_sum);
    println!("Task 2: {}", packet.eval());
}
