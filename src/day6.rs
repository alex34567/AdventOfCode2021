static INPUT: &str = include_str!("input/Day6.txt");

#[derive(Copy, Clone)]
struct DayInfo {
    to_add: u64,
    fish_count: u64,
}

pub fn calc_ans(days: i32) -> u64 {
    let mut cycle = [DayInfo {
        to_add: 0,
        fish_count: 0,
    }; 7];

    for raw_timer in INPUT.trim().split(',') {
        let timer = raw_timer.parse::<usize>().unwrap();
        cycle[timer].fish_count += 1;
    }

    let mut current_day = 0;
    for _ in 0..days {
        let sexual_mature_day = (current_day + 2) % 7;
        cycle[sexual_mature_day].to_add = cycle[current_day].fish_count;
        cycle[current_day].fish_count += cycle[current_day].to_add;
        cycle[current_day].to_add = 0;
        current_day = (current_day + 1) % 7;
    }

    cycle.iter().map(|x| x.fish_count + x.to_add).sum()
}

pub fn day6() {
    println!("Part1: {}", calc_ans(80));
    println!("Part2: {}", calc_ans(256));
}
