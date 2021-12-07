use lazy_static::lazy_static;
use regex::Regex;
use std::mem::swap;

static INPUT: &str = include_str!("input/Day5.txt");

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
}

fn calc_ans(is_part2: bool) -> usize {
    let mut grid = vec![vec![0; 1024]; 1024];
    for line in INPUT.trim().split('\n') {
        let captures = REGEX.captures(line).unwrap();
        let mut from_x = captures[1].parse::<usize>().unwrap();
        let mut from_y = captures[2].parse::<usize>().unwrap();
        let mut to_x = captures[3].parse::<usize>().unwrap();
        let mut to_y = captures[4].parse::<usize>().unwrap();

        if from_x == to_x {
            if from_y > to_y {
                swap(&mut from_y, &mut to_y);
            }

            for line in grid.iter_mut().take(to_y + 1).skip(from_y) {
                line[from_x] += 1;
            }
        } else if from_y == to_y {
            if from_x > to_x {
                swap(&mut from_x, &mut to_x);
            }

            for x in from_x..=to_x {
                grid[from_y][x] += 1;
            }
        } else if is_part2 {
            let x_iter: Box<dyn Iterator<Item = usize>> = if from_x < to_x {
                Box::new(from_x..=to_x)
            } else {
                Box::new((to_x..=from_x).rev())
            };

            let y_iter: Box<dyn Iterator<Item = usize>> = if from_y < to_y {
                Box::new(from_y..=to_y)
            } else {
                Box::new((to_y..=from_y).rev())
            };

            for (x, y) in x_iter.zip(y_iter) {
                grid[y][x] += 1;
            }
        }
    }
    grid.iter().flatten().filter(|x| **x > 1).count()
}

pub fn day5() {
    println!("Part1: {}", calc_ans(false));
    println!("Part2: {}", calc_ans(true));
}
