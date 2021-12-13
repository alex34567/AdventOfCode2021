static INPUT: &str = include_str!("input/Day7.txt");

fn calc_ans(mut distance_to_fuel: impl FnMut(i32) -> i32) -> i32 {
    let ships: Vec<i32> = INPUT
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let min = *ships.iter().min().unwrap();
    let max = *ships.iter().max().unwrap();
    (min..max)
        .map(|i| {
            let mut fuel_total = 0;
            for ship in &ships {
                fuel_total += distance_to_fuel((*ship - i).abs());
            }
            fuel_total
        })
        .min()
        .unwrap()
}

pub fn day7() {
    println!("Part1: {}", calc_ans(|x| x));
    println!("Part2: {}", calc_ans(|x| x * (x + 1) / 2));
}
