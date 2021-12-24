use super::INPUT;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::mem::swap;
use std::str::FromStr;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^Player \d starting position: (\d)$").unwrap();
    static ref OFFSET_TABLE: [u64; 10] = {
        let mut ret = [0; 10];
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    ret[i + j + k] += 1;
                }
            }
        }
        ret
    };
}

struct Player {
    universes: [[u64; 21]; 10],
}

impl Player {
    fn take_turn(&mut self) -> u64 {
        let mut new_universes = [[0; 21]; 10];
        let mut won_unis = 0;
        for (possition, uni_possition) in self.universes.iter().enumerate() {
            for (score, universe_cnt) in uni_possition.iter().enumerate() {
                let mut curr_pos = possition;
                curr_pos += 2;
                for offset in 3..=9 {
                    curr_pos += 1;
                    curr_pos %= 10;
                    let new_score = score + curr_pos + 1;
                    let new_universe_cnt = *universe_cnt * OFFSET_TABLE[offset];
                    if new_score >= 21 {
                        won_unis += new_universe_cnt;
                    } else {
                        new_universes[curr_pos][new_score] += new_universe_cnt;
                    }
                }
            }
        }
        self.universes = new_universes;
        won_unis
    }

    fn count(&self) -> u64 {
        self.universes.iter().flatten().sum::<u64>()
    }

    fn all_won(&self) -> bool {
        self.count() == 0
    }
}

impl FromStr for Player {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        let position = REGEX.captures(string).unwrap()[1].parse::<usize>().unwrap() - 1;
        let mut universes = [[0; 21]; 10];
        universes[position][0] = 1;
        Ok(Self { universes })
    }
}

pub fn day21p2() {
    let mut players = Vec::new();
    for line in INPUT.trim().split('\n') {
        players.push(line.parse::<Player>().unwrap());
    }

    let (player1_list, player2_list) = players.split_at_mut(1);
    let mut player1 = &mut player1_list[0];
    let mut player1_wins = 0;
    let mut player2 = &mut player2_list[0];
    let mut player2_wins = 0;

    loop {
        let win_canidites = player1.take_turn();
        player1_wins += win_canidites * player2.count();
        if player1.all_won() {
            break;
        }
        swap(&mut player1, &mut player2);
        swap(&mut player1_wins, &mut player2_wins);
    }

    println!("Part2: {}", max(player1_wins, player2_wins));
}
