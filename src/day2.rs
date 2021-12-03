use std::str::FromStr;

static INPUT: &str = include_str!("input/Day2.txt");

fn day2_part1() {
    let mut x = 0;
    let mut y = 0;
    for line in INPUT.split('\n') {
        if line.is_empty() {
            continue;
        }
        let words = line.split(' ').collect::<Vec<_>>();
        assert_eq!(words.len(), 2);
        let num = i32::from_str(words[1]).unwrap();
        match words[0] {
            "forward" => x += num,
            "down" => y += num,
            "up" => y -= num,
            _ => panic!(),
        }
    }
    println!("Part1: {}", x * y);
}

fn day2_part2() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in INPUT.split('\n') {
        if line.is_empty() {
            continue;
        }
        let words = line.split(' ').collect::<Vec<_>>();
        assert_eq!(words.len(), 2);
        let num = i32::from_str(words[1]).unwrap();
        match words[0] {
            "forward" => {
                x += num;
                y += num * aim;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => panic!(),
        }
    }
    println!("Part2: {}", x * y);
}

pub fn day2() {
    day2_part1();
    day2_part2();
}
