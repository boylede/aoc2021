use std::slice::Iter;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day16.txt");

const INPUT_A: &str = "D2FE28";
const INPUT_B: &str = "38006F45291200";
const SUB_B: &str = "11010001010";

const INPUT_C: &str = "EE00D40C823060";

const INPUT_D: &str = "8A004A801A8002F478";

const INPUT_E: &str = "620080001611562C8802118E34";

const INPUT_F: &str = "C0015000016115A2E0802F182340";
const INPUT_G: &str = "A0016C880162017C3686B18A3D4780";

const INPUT_H: &str ="C200B40A82";

const INPUT_HH: &str ="04005AC33890";
const INPUT_I: &str ="880086C3E88112";

const INPUT_J: &str ="CE00C43D881120";


const INPUT_K: &str ="D8005AC2A8F0";

const INPUT_L: &str ="F600BC2D8F";

const INPUT_M: &str ="9C005AC2F8F0";

const INPUT_N: &str ="9C0141080250320F1802104A08";

// 93889017207


fn main() {
    println!("day 16...");



    /////////////////// binary input
    // let bit_bytes: Vec<u8> = SUB_B.chars().map(|c|c.to_digit(2).unwrap() as u8).rev().collect();
    // let bits = BitIter::raw(bit_bytes);
    //////////////////// hexadecimal input
    let bit_bytes: Vec<u8> = INPUT
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .rev()
        .collect();

        let mut bits = BitIter::new(bit_bytes.clone());
        let packet = bits.take_packet().unwrap();
        println!("packet: {}", packet.format());
    {
        ////////////// binary input
        // let mut bits = bits.clone();
        /////////////// hex input
        let mut bits = BitIter::new(bit_bytes.clone());



        // let whole: Vec<u8> = bits.iter().copied().collect();
        // println!("{:?}", whole);
        // let packet = bits.take_packet().unwrap();
        // println!("packet: {:?}", packet);
        let version_sum = packet.sum_version();
        // for b in whole.iter();

        // let mut iter = bits.iter();

        // // let rest: Vec<u8> = iter.clone().copied().collect();
        // // println!("rest: {:?}", rest);
        // let version = take(&mut iter, 3);
        // // .by_ref()
        // // .take(3)
        // // .collect::<Vec<_>>()
        // // .iter().rev()
        // // .inspect(|i| println!("took {}", i))
        // // .copied()
        // // // .rev()
        // // .enumerate()
        // // .fold(0, |whole: u32, (i, &bit)| whole | (bit as u32) << i);

        // // let rest: Vec<u8> = iter.copied().collect();
        // println!("version: {:?}", version);
        // // println!("rest: {:?}", rest);

        // let packet_type = take(&mut iter, 3);
        // // iter
        // //     .by_ref()
        // //     .take(3)
        // //     .inspect(|i| println!("{}", i))
        // //     .copied()
        // //     .rev()
        // //     .enumerate()
        // //     .fold(0, |whole: u32, (i, bit)| whole | (bit as u32) << i);
        // println!("packet_type: {:?}", packet_type);

        // match packet_type {
        //     4 => {
        //         let mut last = take(&mut iter, 5);
        //         let mut integer = 0;
        //         // let mut i = 0;
        //         // println!("got {}", last);
        //         while (last >> 4) & 0b1 == 1 {
        //             last = last & 0b1111;

        //             integer = integer << 4;

        //             integer |= last;

        //             last = take(&mut iter, 5);
        //         }

        //         // last = last & 0b1111;

        //         integer = integer << 4;
        //         integer |= last;

        //         println!("got number: {}", integer);
        //         //  = iter.by_ref().take_while(|a| {
        //         //     a >> 7
        //         // })
        //     }
        //     _ => {
        //         // an operator
        //         let length_type_id = take(&mut iter, 1);
        //         if length_type_id == 0 {

        //             // 15
        //             let total_bit_length = take(&mut iter, 15);
        //             println!("operator with bit length: {}", total_bit_length);
        //         } else {
        //             // 11
        //             let packet_count = take(&mut iter, 11);
        //             println!("operator with packets: {}", packet_count);
        //         }

        //         // println!("Unexpected packet type: {}", packet_type);
        //     }
        // }

        println!("parta: {}", version_sum);
    }
    {
        
        // println!("packet: {:?}", packet);
        let result = packet.evaluate();
        println!("partb: {}", result);
    }
}

// fn take(iter: &mut Iter<u8>, count: usize) -> u32 {
//     iter
//         // .by_ref()
//         .take(count)
//         .collect::<Vec<_>>()
//         .iter()
//         .rev()
//         // .inspect(|i| println!("took {}", i))
//         .copied()
//         // .rev()
//         .enumerate()
//         .fold(0, |whole: u32, (i, &bit)| whole | (bit as u32) << i)
// }

#[derive(Debug, Clone)]
pub struct BitIter {
    inner: Vec<u8>,
    current: Vec<u8>,
}

impl BitIter {
    fn raw(current: Vec<u8>) -> BitIter {
        BitIter {
            inner: Vec::with_capacity(0),
            current,
        }
    }
    fn new(inner: Vec<u8>) -> BitIter {
        BitIter {
            inner,
            current: Vec::with_capacity(4),
        }
    }
    fn next(&mut self) -> Option<u8> {
        if self.current.len() == 0 {
            if let Some(byte) = self.inner.pop() {
                // println!("emptied 4-bit buffer, got more: {:x}", byte);
                let slice = [
                    byte >> 0 & 0b1,
                    byte >> 1 & 0b1,
                    byte >> 2 & 0b1,
                    byte >> 3 & 0b1,
                ];
                // println!("{:?}", slice);
                self.current.extend_from_slice(&slice);
            } else {
                return None;
            }
        }
        self.current.pop()
    }
    fn take_int(&mut self, count: usize) -> Option<u64> {
        // println!("taking {} bits", count);
        let mut buf = Vec::with_capacity(count);
        for _ in 0..count {
            let bit = self.next()?;
            // println!("bit {}", bit);
            buf.push(bit);
        }
        let num = buf
            .iter()
            .rev()
            .enumerate()
            .fold(0, |whole, (i, &bit)| whole | (bit as u64) << i);
        // println!("got number {} out of {} bits: {:?}", num, count, buf);
        Some(num)
    }
    fn take_varint(&mut self) -> Option<u64> {
        let mut last = self.take_int(5)?;
        let mut integer = 0;

        while (last >> 4) & 0b1 == 1 {
            last = last & 0b1111;

            integer = integer << 4;

            integer |= last;

            last = self.take_int(5).unwrap();
        }

        integer = integer << 4;
        integer |= last;

        // println!("got number: {}", integer);

        Some(integer)
    }
    fn take_nested(&mut self, count: u32) -> Option<BitIter> {
        // println!("separating out {} bits", count);
        let mut buf = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let bit = self.next()?;
            // print!("{}", bit);
            buf.push(bit);
        }
        // println!("got {} bits: {:?}", count, buf);

        Some(BitIter::raw(buf.iter().rev().copied().collect()))
    }
    fn is_empty(&self) -> bool {
        self.inner.len() == 0 && self.current.len() == 0
    }
    fn take_packets(&mut self) -> Option<Vec<Packet>> {
        // take packets until exhausted
        let mut packets = Vec::new();
        while !self.is_empty() {
            let p = self.take_packet()?;
            // println!("took packet {:?}", p);
            packets.push(p);
        }
        Some(packets)
    }
    fn take_packet(&mut self) -> Option<Packet> {
        let version = self.take_int(3)? as u32;
        let kind = self.take_int(3)? as u32;
        // println!("taking packet with version {} and kind {}", version, kind);
        let inner = if kind == 4 {
            let number = self.take_varint()?;
            PacketInner::Literal(number)
        } else {
            let encoding = self.next()?;
            if encoding == 0 {
                let length = self.take_int(15)?;
                // println!("looking for all packets in {} bits", length);
                let mut child = self.take_nested(length as u32)?;
                let packets = child.take_packets()?;
                // println!("found {} packets", packets.len());
                PacketInner::Parent(encoding, length as u32, packets)
            } else {
                let count = self.take_int(11)? as u32;
                // println!("looking for {} packets", count);
                let packets: Vec<Packet> =
                    (0..count).map(|_| self.take_packet().unwrap()).collect();
                // println!("found {} packets", packets.len());
                PacketInner::Parent(encoding, count, packets)
            }
        };

        Some(Packet {
            version,
            kind,
            inner,
        })
    }
}

#[derive(Debug, Clone)]
struct Packet {
    version: u32,
    kind: u32,
    inner: PacketInner,
    // length_encoding: u8,
    // length: u32,
    // children: Vec<Packet>,
}

impl Packet {
    fn format(&self) -> String {
        match self.inner {
            PacketInner::Literal(lit) => format!("{}", lit),
            PacketInner::Parent(_,_,ref children) => {
                match self.kind {
                    0 => {
                        // sum
                      let s: String = children.iter().map(|p|p.format()).intersperse(" + ".to_string()).collect();
                        // let a = iter.next().unwrap();
                        // let b = iter.next().unwrap();
                        format!("({})", s)
                        
                    },
                    1 => {
                        // product
                        let s: String =children.iter().map(|p|p.format()).intersperse(" * ".to_string()).collect();
                        // children.iter().map(|p|p.evaluate()).product()
                        format!("({})", s)
                    },
                    2 => {
                        // min
                        // children.iter().map(|p|p.evaluate()).min().unwrap()

                        let s: String = children.iter().map(|p|p.format()).intersperse(" , ".to_string()).collect();
                        format!("min({})",s)
                    },
                    3 => {
                        // max
                        // children.iter().map(|p|p.evaluate()).max().unwrap()
                        let s: String = children.iter().map(|p|p.format()).intersperse(" , ".to_string()).collect();
                        format!("max({})",s)
                    },
                    4 => {
                        panic!("should be unreachable");
                    },
                    5 => {
                        assert!(children.len() == 2);
                        // greater than
                        // children.iter().map(|p|p.evaluate()).reduce(|a, b| {
                        //     if a > b {
                        //         1
                        //     } else {
                        //         0
                        //     }
                        // }).unwrap()
                        let mut iter = children.iter().map(|p|p.format());
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        format!("({} > {})", a, b)
                        
                    },
                    6 => {
                        // less than
                        assert!(children.len() == 2);
                        // children.iter().map(|p|p.evaluate()).reduce(|a, b| {
                        //     if a < b {
                        //         1
                        //     } else {
                        //         0
                        //     }
                        // }).unwrap()
                        let mut iter = children.iter().map(|p|p.format());
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        format!("({} < {})", a, b)
                    },
                    7 => {
                        // equal to
                        assert!(children.len() == 2);
                        // children.iter().map(|p|p.evaluate()).reduce(|a, b| {
                        //     if a == b {
                        //         1
                        //     } else {
                        //         0
                        //     }
                        // }).unwrap()
                        let mut iter = children.iter().map(|p|p.format());
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        format!("({} == {})", a, b)
                    },
                    _=> {
                        unimplemented!()
                    }
                }
                // self.version + children.iter().map(|c|c.sum_version()).sum::<u32>()
            }
        }

        // unimplemented!()
    }
    fn sum_version(&self) -> u32 {
        match self.inner {
            PacketInner::Literal(_) => self.version,
            PacketInner::Parent(_,_,ref children) => {
                self.version + children.iter().map(|c|c.sum_version()).sum::<u32>()
            }
        }
    }
    fn evaluate(&self) -> u64 {
        match self.inner {
            PacketInner::Literal(lit) => lit as u64,
            PacketInner::Parent(_,_,ref children) => {
                match self.kind {
                    0 => {
                        // sum
                        children.iter().map(|p|p.evaluate()).sum()
                        
                    },
                    1 => {
                        // product
                        children.iter().map(|p|p.evaluate()).product()
                    },
                    2 => {
                        // min
                        children.iter().map(|p|p.evaluate()).min().unwrap()
                    },
                    3 => {
                        // max
                        children.iter().map(|p|p.evaluate()).max().unwrap()
                    },
                    4 => {
                        panic!("should be unreachable");
                    },
                    5 => {
                        assert!(children.len() == 2);
                        // greater than
                        let mut iter = children.iter().map(|p|p.evaluate());
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        // children.iter().map(|p|p.evaluate()).reduce(|a, b| {
                            if a > b {
                                1
                            } else {
                                0
                            }
                        // }).unwrap()
                        
                    },
                    6 => {
                        // less than
                        assert!(children.len() == 2);
                        let mut iter = children.iter().map(|p|p.evaluate());
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        // children.iter().map(|p|p.evaluate()).reduce(|a, b| {
                            if a < b {
                                1
                            } else {
                                0
                            }
                        // }).unwrap()
                    },
                    7 => {
                        // equal to
                        assert!(children.len() == 2);
                        let mut iter = children.iter().map(|p|p.evaluate());
                        let a = iter.next().unwrap();
                        let b = iter.next().unwrap();
                        // children.iter().map(|p|p.evaluate()).reduce(|a, b| {
                            if a == b {
                                1
                            } else {
                                0
                            }
                        // }).unwrap()
                    },
                    _=> {
                        unimplemented!()
                    }
                }
                // self.version + children.iter().map(|c|c.sum_version()).sum::<u32>()
            }
        }
    }
}

#[derive(Debug, Clone)]
enum PacketInner {
    Literal(u64),
    Parent(u8, u32, Vec<Packet>),
}
