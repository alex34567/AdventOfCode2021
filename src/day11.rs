use crate::util::{grid_parse, two_d_diagonal_adjacent_enumerated_iter_mut as adj_enu_iter_mut};

struct Octo {
    energy: u8,
    has_flashed: bool,
}

static INPUT: &str = include_str!("input/Day11.txt");

pub fn day11() {
    let mut grid = grid_parse(INPUT, |energy| Octo {
        energy,
        has_flashed: false,
    });

    let mut part1 = 0;
    let mut part2 = 0;

    let mut i = 0;
    loop {
        let mut flashed_now = 0;
        for octo in grid.iter_mut().flatten() {
            octo.energy += 1;
            octo.has_flashed = false;
        }

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let mut flash_stack = vec![(x, y)];
                while let Some((x, y)) = flash_stack.pop() {
                    if !grid[y][x].has_flashed && grid[y][x].energy > 9 {
                        flashed_now += 1;
                        if i < 100 {
                            part1 += 1;
                        }
                        grid[y][x].has_flashed = true;
                        for (x, y, octo) in adj_enu_iter_mut(&mut grid, x, y) {
                            octo.energy += 1;
                            flash_stack.push((x, y));
                        }
                    }
                }
            }
        }

        for octo in grid.iter_mut().flatten() {
            if octo.has_flashed {
                octo.energy = 0;
            }
        }

        if grid.len() * grid[0].len() == flashed_now {
            part2 = i;
        }

        if i >= 100 && part2 != 0 {
            break;
        }
        i += 1;
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2 + 1);
}
