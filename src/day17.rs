use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;

static INPUT: &str = include_str!("input/Day17.txt");

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^target area: x=(\d+)..(\d+), y=(-?\d+)..(-?\d+)$").unwrap();
}

pub fn day17() {
    let captures = REGEX.captures(INPUT.trim()).unwrap();
    let x_min: i32 = captures[1].parse().unwrap();
    let x_max: i32 = captures[2].parse().unwrap();
    let y_min: i32 = captures[3].parse().unwrap();
    let y_max: i32 = captures[4].parse().unwrap();

    let mut steps = 1;
    let mut y_highest = 0;
    let mut possible_vel = Vec::new();
    while steps < 1000 {
        let mut y_hits = Vec::new();
        let mut inital_y_velocity = y_min - 1;
        while inital_y_velocity < 1000 {
            let mut y_pos = 0;
            let mut y_velocity = inital_y_velocity;
            let mut local_highest = 0;
            for _ in 0..steps {
                y_pos += y_velocity;
                local_highest = max(local_highest, y_pos);
                y_velocity -= 1;
            }
            if (y_min..=y_max).contains(&y_pos) {
                y_highest = max(local_highest, y_highest);
                y_hits.push(inital_y_velocity);
            }

            inital_y_velocity += 1;
        }
        let mut x_hits = Vec::new();
        if !y_hits.is_empty() {
            let mut init_x_velocity: i32 = 1;
            loop {
                let mut x_pos = 0;
                let mut x_velocity = init_x_velocity;
                for _ in 0..steps {
                    x_pos += x_velocity;
                    x_velocity -= x_velocity.signum();
                }
                if (x_min..=x_max).contains(&x_pos) {
                    x_hits.push(init_x_velocity);
                }
                let overshot = x_pos > x_max;

                init_x_velocity += 1;
                if overshot {
                    break;
                }
            }
        }
        for y_vel in &y_hits {
            for x_vel in &x_hits {
                possible_vel.push((*y_vel, *x_vel));
            }
        }

        steps += 1;
    }

    possible_vel.sort_unstable();
    possible_vel.dedup();

    println!("Part1: {}", y_highest);
    println!("Part2: {}", possible_vel.len());
}
