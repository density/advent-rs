use std::str::from_utf8;

use nom::bits::complete::tag;
use nom::bits::complete::take;
use nom::multi::{many0, many_m_n, many_till};
use nom::sequence::tuple;
use nom::IResult;

use hymns::runner::timed_run;

macro_rules! read_bits {
    ($name:ident, $bits:expr, $return_type:ty) => {
        fn $name(input: BitIO) -> IResult<BitIO, $return_type> {
            take($bits)(input)
        }
    };
}

const INPUT: &str = include_str!("../input.txt");

type BitIO<'a> = (&'a [u8], usize);

#[derive(Debug, Clone, Eq, PartialEq)]
enum PacketContents {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    contents: PacketContents,
}

fn load_packet_from_hex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks(2)
        .map(|pair| {
            let s = from_utf8(pair).unwrap();
            u8::from_str_radix(s, 16).unwrap() << (pair.len() % 2 * 4)
        })
        .collect()
}

read_bits!(read_version, 3usize, u8);
read_bits!(read_packet_type_id, 3usize, u8);
read_bits!(read_length_type_id, 1usize, u8);
read_bits!(read_bit_length, 15usize, usize);
read_bits!(read_packet_count, 11usize, usize);

fn header(input: BitIO) -> IResult<BitIO, (u8, u8)> {
    tuple((read_version, read_packet_type_id))(input)
}

fn non_terminal_literal_group(input: BitIO) -> IResult<BitIO, u8> {
    let (input, _) = tag(1, 1usize)(input)?;
    take(4usize)(input)
}

fn terminal_literal_group(input: BitIO) -> IResult<BitIO, u8> {
    let (input, _) = tag(0, 1usize)(input)?;
    take(4usize)(input)
}

fn literal_group(input: BitIO) -> IResult<BitIO, (Vec<u8>, u8)> {
    many_till(non_terminal_literal_group, terminal_literal_group)(input)
}

fn literal_value(input: BitIO) -> IResult<BitIO, u64> {
    let (input, (g1, g2)) = literal_group(input)?;

    let mut value = 0;

    for segment in g1 {
        value <<= 4;
        value |= u64::from(segment);
    }

    value <<= 4;
    value |= u64::from(g2);

    Ok((input, value))
}

fn operator_packet(input: (&[u8], usize)) -> IResult<BitIO, Vec<Packet>> {
    let (input, length_type_id) = read_length_type_id(input)?;

    match length_type_id {
        0 => {
            let ((data_before_reading_packets, offset_before_reading_packets), bit_length) =
                read_bit_length(input)?;

            // Calculate the least number of bytes we have to read to encompass bit_length bits. If nom
            // supported length_value with bits this would be much simpler.
            let last_byte = (offset_before_reading_packets + bit_length) / 8 + 1;

            let ((data_after_reading_packets, offset_after_reading_packets), packets) =
                many0(packet)((
                    &data_before_reading_packets[..last_byte],
                    offset_before_reading_packets,
                ))?;

            let bytes_advanced = (data_after_reading_packets.as_ptr() as usize)
                - (data_before_reading_packets.as_ptr() as usize);

            let total_bits_read =
                bytes_advanced * 8 - offset_before_reading_packets + offset_after_reading_packets;

            debug_assert!(total_bits_read == bit_length);

            Ok((
                (
                    &data_before_reading_packets[bytes_advanced..],
                    offset_after_reading_packets,
                ),
                packets,
            ))
        }
        1 => {
            let (input, packet_count) = read_packet_count(input)?;
            many_m_n(packet_count, packet_count, packet)(input)
        }
        _ => unreachable!(),
    }
}

fn packet(input: BitIO) -> IResult<BitIO, Packet> {
    let (input, (version, type_id)) = header(input)?;

    let (input, contents) = if type_id == 4 {
        let (input, literal_value) = literal_value(input)?;

        (input, PacketContents::Literal(literal_value))
    } else {
        let (input, sub_packets) = operator_packet(input)?;

        (input, PacketContents::SubPackets(sub_packets))
    };

    Ok((
        input,
        Packet {
            version,
            type_id,
            contents,
        },
    ))
}

fn sum_version_numbers(packet: &Packet) -> usize {
    match &packet.contents {
        PacketContents::Literal(_) => usize::from(packet.version),
        PacketContents::SubPackets(p) => {
            usize::from(packet.version) + p.iter().fold(0, |acc, p| acc + sum_version_numbers(p))
        }
    }
}

fn calc_packet_value(packet: &Packet) -> u64 {
    match &packet.contents {
        PacketContents::Literal(n) => {
            debug_assert!(packet.type_id == 4);
            *n
        }
        PacketContents::SubPackets(sub_packets) => match packet.type_id {
            0 => {
                debug_assert!(!sub_packets.is_empty());
                sub_packets
                    .iter()
                    .fold(0, |acc, packet| acc + calc_packet_value(packet))
            }
            1 => {
                debug_assert!(!sub_packets.is_empty());
                sub_packets
                    .iter()
                    .fold(1, |acc, packet| acc * calc_packet_value(packet))
            }
            2 => {
                debug_assert!(!sub_packets.is_empty());
                sub_packets
                    .iter()
                    .fold(u64::MAX, |acc, packet| acc.min(calc_packet_value(packet)))
            }
            3 => {
                debug_assert!(!sub_packets.is_empty());
                sub_packets
                    .iter()
                    .fold(u64::MIN, |acc, packet| acc.max(calc_packet_value(packet)))
            }
            5 => {
                debug_assert!(sub_packets.len() == 2);
                if calc_packet_value(&sub_packets[0]) > calc_packet_value(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                debug_assert!(sub_packets.len() == 2);
                if calc_packet_value(&sub_packets[0]) < calc_packet_value(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                debug_assert!(sub_packets.len() == 2);
                if calc_packet_value(&sub_packets[0]) == calc_packet_value(&sub_packets[1]) {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        },
    }
}

/// All packets:
/// Header:
/// 3 bits - version
/// 3 bits - packet type ID
///
/// Type ID == 4: Literal Value
/// 4 groups of 5 bits. First bit is 1 except in last group, which is zero. remaining bits make up number
///
/// Type ID != 4: Operator Packet
/// After header: 1 bit - length type ID
///
/// Length Type ID == 0: Next 15 bits are number that indicates total length of subpackets
/// Length TYpe ID == 1: Next 11 bits are number that indicates total number of sub packets.
fn part1() -> usize {
    let bytes = load_packet_from_hex(INPUT);
    let (_, packet) = packet((bytes.as_slice(), 0)).unwrap();

    sum_version_numbers(&packet)
}

fn part2() -> u64 {
    let bytes = load_packet_from_hex(INPUT);

    let (_, packet) = packet((bytes.as_slice(), 0)).unwrap();

    calc_packet_value(&packet)
}

fn main() {
    timed_run(1, part1);
    timed_run(2, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 871);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 68703010504);
    }

    #[test]
    fn test_read_header() {
        let data = load_packet_from_hex("D2FE28");

        let input = (data.as_slice(), 0);

        let (_, (version, type_id)) = header(input).unwrap();

        assert_eq!(version, 6);
        assert_eq!(type_id, 4);
    }

    #[test]
    fn test_read_literal_group() {
        let data: [u8; 2] = [0b10111111, 0b10001010];
        let input = (&data[..], 0);

        let (_, value) = literal_group(input).unwrap();
        assert_eq!(value, (vec![0b111, 0b1110], 0b101));
    }

    #[test]
    fn test_read_literal() {
        let data = load_packet_from_hex("D2FE28");

        let (_, packet) = packet((data.as_slice(), 0)).unwrap();

        assert_eq!(
            packet,
            Packet {
                version: 6,
                type_id: 4,
                contents: PacketContents::Literal(2021,),
            }
        )
    }

    #[test]
    fn test_read_bit_length() {
        let data: [u8; 2] = [0, 0b11011];
        let input = (&data[..], 1);

        let (_, value) = read_bit_length(input).unwrap();
        assert_eq!(value, 27);
    }

    #[test]
    fn test_example_version_numbers() {
        let (_, packets) = packet((load_packet_from_hex("D2FE28").as_slice(), 0)).unwrap();
        assert_eq!(sum_version_numbers(&packets), 6);

        let (_, packets) = packet((load_packet_from_hex("38006F45291200").as_slice(), 0)).unwrap();
        assert_eq!(sum_version_numbers(&packets), 9);

        let (_, packets) = packet((load_packet_from_hex("EE00D40C823060").as_slice(), 0)).unwrap();
        assert_eq!(sum_version_numbers(&packets), 14);

        let (_, packets) =
            packet((load_packet_from_hex("8A004A801A8002F478").as_slice(), 0)).unwrap();
        assert_eq!(sum_version_numbers(&packets), 16);

        let (_, packets) = packet((
            load_packet_from_hex("620080001611562C8802118E34").as_slice(),
            0,
        ))
        .unwrap();
        assert_eq!(sum_version_numbers(&packets), 12);

        let (_, packets) = packet((
            load_packet_from_hex("C0015000016115A2E0802F182340").as_slice(),
            0,
        ))
        .unwrap();
        assert_eq!(sum_version_numbers(&packets), 23);

        let (_, packets) = packet((
            load_packet_from_hex("A0016C880162017C3686B18A3D4780").as_slice(),
            0,
        ))
        .unwrap();
        assert_eq!(sum_version_numbers(&packets), 31);
    }

    #[test]
    fn test_example_packet_values() {
        let (_, packets) = packet((load_packet_from_hex("C200B40A82").as_slice(), 0)).unwrap();
        assert_eq!(calc_packet_value(&packets), 3);

        let (_, packets) = packet((load_packet_from_hex("04005AC33890").as_slice(), 0)).unwrap();
        assert_eq!(calc_packet_value(&packets), 54);

        let (_, packets) = packet((load_packet_from_hex("880086C3E88112").as_slice(), 0)).unwrap();
        assert_eq!(calc_packet_value(&packets), 7);

        let (_, packets) = packet((load_packet_from_hex("CE00C43D881120").as_slice(), 0)).unwrap();
        assert_eq!(calc_packet_value(&packets), 9);

        let (_, packets) = packet((load_packet_from_hex("D8005AC2A8F0").as_slice(), 0)).unwrap();
        assert_eq!(calc_packet_value(&packets), 1);

        let (_, packets) = packet((load_packet_from_hex("F600BC2D8F").as_slice(), 0)).unwrap();
        assert_eq!(calc_packet_value(&packets), 0);

        let (_, packets) = packet((load_packet_from_hex("9C005AC2F8F0").as_slice(), 0)).unwrap();
        assert_eq!(calc_packet_value(&packets), 0);

        let (_, packets) = packet((
            load_packet_from_hex("9C0141080250320F1802104A08").as_slice(),
            0,
        ))
        .unwrap();
        assert_eq!(calc_packet_value(&packets), 1);
    }
}
