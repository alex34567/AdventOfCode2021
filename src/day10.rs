use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Bracket {
    start: char,
    end: char,
    p1_score: i32,
    p2_score: u64,
}

static INPUT: &str = include_str!("input/Day10.txt");

lazy_static! {
    static ref BRACKET_LOOKUP_REV_LOOKUP: [HashMap<char, Bracket>; 2] = {
        let mut lookup = HashMap::new();
        let mut rev_lookup = HashMap::new();
        let mut add_bracket = |start, end, p1_score, p2_score| {
            let bracket = Bracket {
                start,
                end,
                p1_score,
                p2_score,
            };
            lookup.insert(start, bracket);
            rev_lookup.insert(end, bracket);
        };
        add_bracket('{', '}', 1197, 3);
        add_bracket('(', ')', 3, 1);
        add_bracket('[', ']', 57, 2);
        add_bracket('<', '>', 25137, 4);
        [lookup, rev_lookup]
    };
    static ref START_TO_BRACKET: &'static HashMap<char, Bracket> = &BRACKET_LOOKUP_REV_LOOKUP[0];
    static ref END_TO_BRACKET: &'static HashMap<char, Bracket> = &BRACKET_LOOKUP_REV_LOOKUP[1];
}

pub fn day10() {
    let mut part1 = 0;
    let mut part2_scors = Vec::new();
    'line_loop: for line in INPUT.trim().split('\n') {
        let mut stack = Vec::new();
        for bracket in line.chars() {
            if let Some(bracket_info) = START_TO_BRACKET.get(&bracket) {
                stack.push(bracket_info);
                continue;
            }

            if let Some(expected_info) = END_TO_BRACKET.get(&bracket) {
                let info = stack.pop().unwrap();
                if expected_info != info {
                    part1 += expected_info.p1_score;
                    continue 'line_loop;
                }
            }
        }
        let mut line_score = 0;
        for bracket in stack.into_iter().rev() {
            line_score *= 5;
            line_score += bracket.p2_score;
        }
        part2_scors.push(line_score);
    }

    part2_scors.sort_unstable();
    let part2 = part2_scors[part2_scors.len() / 2];

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
