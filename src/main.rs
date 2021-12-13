use std::env;
use std::str::FromStr;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

const DAY_COUNT: u8 = 9;

static ARG_ERR_STRING: &str = "Put a day number or * for all";

fn exec_day(day_n: u8) {
    match day_n {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        5 => day5::day5(),
        6 => day6::day6(),
        7 => day7::day7(),
        8 => day8::day8(),
        9 => day9::day9(),
        _ => panic!(),
    }
}

fn main() {
    let raw_day_n = env::args().nth(1).expect(ARG_ERR_STRING);
    if raw_day_n == "*" {
        for x in 1..DAY_COUNT + 1 {
            if x != 1 {
                println!();
            }
            print!("Day{}: ", x);
            exec_day(x)
        }
    } else {
        let day_n = u8::from_str(&raw_day_n).expect(ARG_ERR_STRING);
        if !(1..=DAY_COUNT).contains(&day_n) {
            panic!("Day number must be an integer between 1 and {}", DAY_COUNT);
        }
        exec_day(day_n)
    }
}
