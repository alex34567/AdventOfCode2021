use crate::util::BitIter;
use std::cmp::{max, min};

static INPUT: &str = include_str!("input/Day16.txt");

fn parse_packet(
    stream: &mut BitIter<impl Iterator<Item = u8>>,
    combined_version: &mut u32,
    length: &mut u32,
) -> u64 {
    let ver = stream.take_into_u32(3).unwrap();
    let id = stream.take_into_u8(3).unwrap();
    *length += 6;
    *combined_version += ver;
    if id == 4 {
        let mut literal = 0;
        loop {
            let more_groups = stream.next().unwrap();
            literal <<= 4;
            literal |= stream.take_into_u64(4).unwrap();
            *length += 5;
            if !more_groups {
                return literal;
            }
        }
    } else {
        let length_type = stream.next().unwrap();
        *length += 1;
        let (mut bit_len, mut packet_len) = if length_type {
            *length += 11;
            (u32::MAX, stream.take_into_u16(11).unwrap())
        } else {
            *length += 15;
            (stream.take_into_u32(15).unwrap(), u16::MAX)
        };

        if (5..=7).contains(&id) {
            packet_len = 2;
        }

        let mut ret = match id {
            1 => 1,
            2 => u64::MAX,
            _ => 0,
        };

        while bit_len > 0 && packet_len > 0 {
            packet_len -= 1;
            let mut sub_length = 0;
            let sub_expr = parse_packet(stream, combined_version, &mut sub_length);
            match id {
                0 => ret += sub_expr,
                1 => ret *= sub_expr,
                2 => ret = min(ret, sub_expr),
                3 => ret = max(ret, sub_expr),
                5 | 6 | 7 => {
                    if packet_len == 1 {
                        ret = sub_expr
                    } else {
                        ret = (match id {
                            5 => ret > sub_expr,
                            6 => ret < sub_expr,
                            7 => ret == sub_expr,
                            _ => unreachable!(),
                        }) as u64
                    }
                }
                _ => panic!(),
            }

            *length += sub_length;
            bit_len -= sub_length;
        }
        ret
    }
}

pub fn day16() {
    let mut bit_string = Vec::new();
    for mut byte in INPUT.trim().as_bytes().chunks(2) {
        let padded_byte;
        if byte.len() == 1 {
            padded_byte = [byte[0], b'0'];
            byte = &padded_byte;
        }
        let hex_byte = std::str::from_utf8(byte).unwrap();
        bit_string.push(u8::from_str_radix(hex_byte, 16).unwrap());
    }

    let mut combined_version = 0;
    let mut length = 0;
    let mut stream = BitIter::new(bit_string.iter().copied());
    let value = parse_packet(&mut stream, &mut combined_version, &mut length);

    println!("Part1: {}", combined_version);
    println!("Part2: {}", value);
}
