use crate::util::adjacent_iter::two_d_straight_adjacent_enumerated_iter_mut as td_adj_iter_mut;
use crate::util::grid_parse;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone)]
struct CaveSquare {
    risk: u32,
    distance: u32,
}

static INPUT: &str = include_str!("input/Day15.txt");

struct PathfindHeapData {
    distance: u32,
    x: usize,
    y: usize,
}

impl PartialEq for PathfindHeapData {
    fn eq(&self, rhs: &Self) -> bool {
        self.distance == rhs.distance
    }
}

impl PartialOrd for PathfindHeapData {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for PathfindHeapData {
    fn cmp(&self, rhs: &Self) -> Ordering {
        rhs.distance.cmp(&self.distance)
    }
}

impl Eq for PathfindHeapData {}

fn calc_ans(mut grid: Vec<Vec<CaveSquare>>) -> u32 {
    grid[0][0].distance = 0;

    let mut visit_heap = BinaryHeap::new();
    visit_heap.push(PathfindHeapData {
        distance: 0,
        x: 0,
        y: 0,
    });

    while let Some(node) = visit_heap.pop() {
        if grid[node.y][node.x].distance != node.distance {
            continue;
        }
        let next_distance = node.distance + grid[node.y][node.x].risk;
        for (x, y, adj_square) in td_adj_iter_mut(&mut grid, node.x, node.y) {
            if adj_square.distance > next_distance {
                adj_square.distance = next_distance;
                visit_heap.push(PathfindHeapData {
                    x,
                    y,
                    distance: next_distance,
                });
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
