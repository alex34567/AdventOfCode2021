use crate::util::{grid_parse, two_d_straight_adjacent_enumerated_iter_mut as td_adj_iter_mut};

#[derive(Copy, Clone)]
struct CaveSquare {
    risk: u32,
    distance: u32,
}

static INPUT: &str = include_str!("input/Day15.txt");

fn calc_ans(mut grid: Vec<Vec<CaveSquare>>) -> u32 {
    grid[0][0].distance = 0;

    let mut visit_list = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            visit_list.push((x, y));
        }
    }

    while !visit_list.is_empty() {
        let (i, (next_x, next_y)) = visit_list
            .iter()
            .enumerate()
            .min_by_key(|(_, (x, y))| {
                grid[*y][*x]
                    .distance
            })
            .unwrap();
        let next_x = *next_x;
        let next_y = *next_y;
        visit_list.swap_remove(i);

        let next_distance = grid[next_y][next_x].distance + grid[next_y][next_x].risk;
        for (_, _, adj_square) in td_adj_iter_mut(&mut grid, next_x, next_y) {
            if adj_square.distance > next_distance {
                adj_square.distance = next_distance;
            }
        }
    }

    let last_square = grid.last().unwrap().last().unwrap();

    last_square.distance + last_square.risk - grid[0][0].risk
}

pub fn day15() {
    let tile = grid_parse(INPUT, |risk| CaveSquare {
        risk: risk as u32,
        distance: u32::MAX,
    });

    let part1 = calc_ans(tile.clone());

    let mut grid = Vec::new();
    for i in 0..5 {
        for tile_line in &tile {
            let mut tile_line = tile_line.clone();
            for square in &mut tile_line {
                square.risk += i;
                square.risk -= 1;
                square.risk %= 9;
                square.risk += 1;
            }
            let mut line: Vec<CaveSquare> = Vec::new();
            for _ in 0..5 {
                line.extend(&tile_line);
                for square in &mut tile_line {
                    square.risk += 1;
                    if square.risk > 9 {
                        square.risk = 1;
                    }
                }
            }
            grid.push(line);
        }
    }

    let part2 = calc_ans(grid);

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}
