use crate::util::list_of_integers;

static INPUT: &str = include_str!("input/Day1.txt");

fn day1_part1() {
    let mut iter = list_of_integers(INPUT);
    let mut prev_depth = iter.next().unwrap();
    let mut part1 = 0;
    for num in iter {
        if num > prev_depth {
            part1 += 1;
        }
        prev_depth = num;
    }
    println!("Part1: {}", part1);
}

fn day1_part2() {
    let nums = list_of_integers(INPUT).collect::<Vec<_>>();
    let mut iter = nums.windows(3).map(|window| window.iter().sum::<i32>());
    let mut prev_depth = iter.next().unwrap();
    let mut part2 = 0;
    for depth in iter {
        if depth > prev_depth {
            part2 += 1;
        }
        prev_depth = depth;
    }
    println!("Part2: {}", part2);
}

pub fn day1() {
    day1_part1();
    day1_part2();
}
