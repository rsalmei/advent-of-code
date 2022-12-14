use crate::Input;
use Outcome::*;
use RPS::*;

#[derive(Copy, Clone, PartialEq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn from(letter: u8, map: &str) -> Self {
        let map = map.as_bytes();
        match *map {
            [x, _, _] if x == letter => Rock,
            [_, x, _] if x == letter => Paper,
            [_, _, x] if x == letter => Scissors,
            _ => unreachable!(),
        }
    }

    fn score(self, other: RPS) -> u32 {
        self as u32
            + match (self, other) {
                _ if self == other => 3,
                _ if self == other.reverse(Win) => 6,
                _ => 0,
            }
    }

    fn reverse(self, outcome: Outcome) -> Self {
        let (lose, win) = match (self, outcome) {
            (x, Draw) => return x,
            (Rock, _) => (Scissors, Paper),
            (Paper, _) => (Rock, Scissors),
            (Scissors, _) => (Paper, Rock),
        };
        (outcome == Win).then_some(win).unwrap_or(lose)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Outcome {
    fn from(letter: u8) -> Self {
        match letter {
            b'X' => Lose,
            b'Y' => Draw,
            b'Z' => Win,
            _ => unreachable!(),
        }
    }
}

pub fn run(input: Input) {
    let data = input.as_lines();

    // part one.
    let games = data
        .iter()
        .map(|&x| x.as_bytes())
        .map(|x| (x[0], x[2]))
        .map(|(a, b)| (RPS::from(a, "ABC"), RPS::from(b, "XYZ")))
        .map(|(a, b)| b.score(a));
    println!("{}", games.sum::<u32>());

    // part two.
    let games = data
        .iter()
        .map(|&x| x.as_bytes())
        .map(|x| (x[0], x[2]))
        .map(|(a, b)| (RPS::from(a, "ABC"), Outcome::from(b)))
        .map(|(a, out)| a.reverse(out).score(a));
    println!("{}", games.sum::<u32>());
}
