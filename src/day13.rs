use crate::util::print_grid;
use lazy_static::lazy_static;
use regex::Regex;
use std::mem::swap;

lazy_static! {
    static ref DOT_REGEX: Regex = Regex::new(r"^(\d+),(\d+)$").unwrap();
    static ref FOLD_REGEX: Regex = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
}

static INPUT: &str = include_str!("input/Day13.txt");

pub fn day13() {
    let mut section_iter = INPUT.trim().split("\n\n");
    let dot_section = section_iter.next().unwrap().trim().split('\n').map(|line| {
        let captures = DOT_REGEX.captures(line).unwrap();
        let x = captures[1].parse::<usize>().unwrap();
        let y = captures[2].parse::<usize>().unwrap();
        (x, y)
    });
    let fold_section = section_iter.next().unwrap().trim().split('\n').map(|line| {
        let captures = FOLD_REGEX.captures(line).unwrap();
        let is_y_fold = captures[1] == *"y";
        let fold_point = captures[2].parse::<usize>().unwrap();

        (fold_point, is_y_fold)
    });
    assert!(section_iter.next().is_none());
    let mut x_size = 0;
    let mut y_size = 0;

    for (x, y) in dot_section.clone() {
        if x > x_size {
            x_size = x;
        }

        if y > y_size {
            y_size = y;
        }
    }

    let mut grid = vec![vec![false; x_size + 1]; y_size + 1];

    for (x, y) in dot_section {
        grid[y][x] = true;
    }

    let mut first_fold = true;
    let mut part1 = 0;

    for (mut fold_point, is_y_fold) in fold_section {
        if is_y_fold {
            let mut prepend = Vec::new();
            let up_half_len = fold_point;
            let down_half_len = grid.len() - fold_point - 1;
            if down_half_len > up_half_len {
                let difference = down_half_len - up_half_len;
                prepend = grid.drain(grid.len() - difference..).rev().collect();
            }

            let mut grid_ref: &mut [Vec<bool>] = &mut grid;
            if down_half_len < up_half_len {
                let difference = up_half_len - down_half_len;
                grid_ref = &mut grid_ref[difference..];
                fold_point -= difference;
            }

            let (up_half, down_half_fold) = grid_ref.split_at_mut(fold_point);
            let down_half = &mut down_half_fold[1..];
            for (up_dot, down_dot) in up_half
                .iter_mut()
                .flatten()
                .zip(down_half.iter_mut().rev().flatten())
            {
                *up_dot = *up_dot || *down_dot;
            }

            grid.truncate(up_half_len);

            prepend.append(&mut grid);
            swap(&mut prepend, &mut grid);
        } else {
            let left_half_len = fold_point;
            let right_half_len = grid[0].len() - fold_point - 1;
            if right_half_len < left_half_len {
                let difference = left_half_len - right_half_len;
                fold_point -= difference;
            }

            for line in grid.iter_mut() {
                let mut prepend = Vec::new();
                if right_half_len > left_half_len {
                    let difference = right_half_len - left_half_len;
                    prepend = line.drain(line.len() - difference..).rev().collect();
                }

                let mut line_ref: &mut [bool] = line;
                if right_half_len < left_half_len {
                    let difference = left_half_len - right_half_len;
                    line_ref = &mut line_ref[difference..];
                }

                let (left_half, right_half_fold) = line_ref.split_at_mut(fold_point);
                let right_half = &mut right_half_fold[1..];
                for (left_dot, right_dot) in left_half.iter_mut().zip(right_half.iter_mut().rev()) {
                    *left_dot = *left_dot || *right_dot;
                }

                line.truncate(left_half_len);

                prepend.append(line);
                swap(&mut prepend, line);
            }
        }

        if first_fold {
            part1 = grid.iter().flatten().map(|x| *x as u32).sum();
            first_fold = false;
        }
    }

    println!("Part1: {}", part1);
    println!("Part2:");
    print_grid(&grid, |dot| if *dot { '#' } else { ' ' });
}
