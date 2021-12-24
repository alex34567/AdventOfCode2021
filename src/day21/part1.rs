use super::INPUT;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^Player \d starting position: (\d)$").unwrap();
}

struct Player {
    position: u32,
    score: u32,
}

impl Player {
    fn has_won(&self) -> bool {
        self.score >= 1000
    }

    fn take_turn(&mut self, dice: &mut impl Iterator<Item = u32>) {
        for _ in 0..3 {
            self.position += dice.next().unwrap();
        }
        self.position %= 10;
        self.score += self.position + 1;
    }
}

impl FromStr for Player {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, ()> {
        let position = REGEX.captures(string).unwrap()[1].parse::<u32>().unwrap() - 1;
        Ok(Self { position, score: 0 })
    }
}

pub fn day21p1() {
    let mut players = Vec::new();
    for line in INPUT.trim().split('\n') {
        players.push(line.parse::<Player>().unwrap());
    }

    let mut dice_rolls = 0;
    let mut dice = (1..=100).cycle().map(|n| {
        dice_rolls += 1;
        n
    });

    while players.iter().all(|p| !p.has_won()) {
        for player in players.iter_mut() {
            player.take_turn(&mut dice);
            if player.has_won() {
                break;
            }
        }
    }

    let mut part1 = dice_rolls;

    for player in players {
        if !player.has_won() {
            part1 *= player.score;
        }
    }

    println!("Part1: {}", part1);
}
