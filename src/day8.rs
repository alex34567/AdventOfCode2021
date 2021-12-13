static INPUT: &str = include_str!("input/Day8.txt");
static NUM_TO_SEGMENT: [u8; 10] = [0x77, 0x24, 0x5D, 0x6D, 0x2E, 0x6B, 0x7B, 0x25, 0x7F, 0x6F];

fn to_bitmask(in_digit: &str) -> u8 {
    let mut ret = 0;
    for letter in in_digit.chars() {
        let raw_letter = letter as u32;
        let bit = 1 << (raw_letter - 'a' as u32);
        ret |= bit
    }
    ret
}

pub fn day8() {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in INPUT.trim().split('\n') {
        let mut line_parts = line.split('|');
        let input = line_parts
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(to_bitmask)
            .collect::<Vec<_>>();
        let output = line_parts.next().unwrap().trim().split(' ').map(to_bitmask);
        assert!(line_parts.next().is_none());
        let wire_cf = input.iter().find(|digit| digit.count_ones() == 2).unwrap();
        let wire_bd = input.iter().find(|digit| digit.count_ones() == 4).unwrap() & !wire_cf;
        let wire_a = input.iter().find(|digit| digit.count_ones() == 3).unwrap() & !wire_cf;
        let wire_g = input
            .iter()
            .filter(|digit| ![2, 3, 4].contains(&digit.count_ones()))
            .fold(!wire_a, |acc, digit| acc & digit);
        let wire_e = 0x7F & !(wire_cf | wire_bd | wire_a | wire_g);
        let wire_f = input
            .iter()
            .find(|digit| digit.count_ones() == 6 && (*digit & wire_cf).count_ones() == 1)
            .unwrap()
            & wire_cf;
        let wire_c = wire_cf & !wire_f;
        let wire_b = input
            .iter()
            .find(|digit| digit.count_ones() == 6 && (*digit & wire_bd).count_ones() == 1)
            .unwrap()
            & wire_bd;
        let wire_d = wire_bd & !wire_b;
        let wire_decoder = [wire_a, wire_b, wire_c, wire_d, wire_e, wire_f, wire_g];
        let mut line_value = 0;

        for digit in output {
            let mut unscrambled_display = 0;
            let mut bit = 1;
            for wire in wire_decoder {
                if digit & wire != 0 {
                    unscrambled_display |= bit;
                }
                bit <<= 1;
            }
            let num = NUM_TO_SEGMENT
                .iter()
                .enumerate()
                .find(|(_, x)| unscrambled_display == **x)
                .unwrap()
                .0;
            if [1, 4, 7, 8].contains(&num) {
                part1 += 1;
            }

            line_value *= 10;
            line_value += num;
        }
        part2 += line_value;
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
