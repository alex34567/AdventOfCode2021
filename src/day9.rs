use crate::util::{two_d_adjacent_iter, two_d_adjacent_enumerated_iter_mut as td_adj_enu_iter_mut};

static INPUT: &str = include_str!("input/Day9.txt");

#[derive(Copy, Clone)]
struct HeightMapEntry {
    height: u8,
    is_visited: bool,
}

pub fn day9() {
    let mut heightmap = INPUT
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| {
                    let mut tmp = [0; 4];
                    let height = c.encode_utf8(&mut tmp).parse().unwrap();
                    HeightMapEntry {
                        height,
                        is_visited: height == 9,
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut part1 = 0;

    for (y, line) in heightmap.iter().enumerate() {
        for (x, height_entry) in line.iter().enumerate() {
            let is_lowest = two_d_adjacent_iter(&heightmap, x, y).all(|adj_entry| adj_entry.height > height_entry.height);
            if is_lowest {
                part1 += (height_entry.height) as u32 + 1;
            }
        }
    }

    println!("Part1: {}", part1);

    let mut basin_sizes = Vec::new();

    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            if heightmap[y][x].is_visited {
                continue;
            }

            heightmap[y][x].is_visited = true;

            let mut visit_stack = vec![(x, y)];

            let mut basin_size = 0;
            while let Some((x, y)) = visit_stack.pop() {
                basin_size += 1;

                for (x, y, entry) in td_adj_enu_iter_mut(&mut heightmap, x, y) {
                    if entry.is_visited {
                        continue;
                    }

                    visit_stack.push((x, y));

                    entry.is_visited = true;
                }

            }

            basin_sizes.push(basin_size);
        }
    }
    basin_sizes.sort();
    let part2 = basin_sizes.iter().rev().take(3).product::<u32>();

    println!("Part2: {}", part2);
}
