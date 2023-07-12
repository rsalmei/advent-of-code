use crate::Input;
use std::collections::{BinaryHeap, VecDeque};
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.lines().collect::<Monkeys>();

    // part one.
    let mut monkeys = data.clone();
    let monkey_business = monkeys.sim(20, 3);
    println!("{monkey_business}");

    // part one.
    let mut monkeys = data;
    let monkey_business = monkeys.sim(10000, 1);
    println!("{monkey_business}");
}

#[derive(Debug, Clone)]
struct Monkeys(Vec<Monkey>);

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    test: Test,
    inspected: u32,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Test {
    div: u64,
    is: usize,
    not: usize,
}

impl Monkeys {
    fn sim(&mut self, rounds: usize, relief: u64) -> u64 {
        let gcd = self.0.iter().fold(1, |acc, m| acc * m.test.div);
        (0..rounds).for_each(|_| {
            for mi in 0..self.0.len() {
                while let Some((item, to)) = self.0[mi].round(relief) {
                    self.0[to].items.push_back(item % gcd);
                }
            }
            // println!("round {r}");
            // monkeys
            //     .iter()
            //     .enumerate()
            //     .for_each(|(i, m)| println!("  monkey {i}: {:?} -> {}", m.items, m.inspected))
        });
        let mut inspected = self
            .0
            .iter()
            .map(|m| m.inspected)
            .collect::<BinaryHeap<_>>();
        inspected.pop().unwrap() as u64 * inspected.pop().unwrap() as u64
    }
}

impl Monkey {
    fn round(&mut self, relief: u64) -> Option<(u64, usize)> {
        let item = self.op.apply(self.items.pop_front()?) / relief;
        let to = match item % self.test.div == 0 {
            true => self.test.is,
            false => self.test.not,
        };
        self.inspected += 1;
        Some((item, to))
    }
}

impl Operation {
    fn apply(&self, item: u64) -> u64 {
        match *self {
            Operation::Add(x) => item + x,
            Operation::Mult(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

impl<'a> FromIterator<&'a str> for Monkeys {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut m = Vec::new();
        let mut it = iter.into_iter();
        while let Some(_monkey) = it.next() {
            let items = it.next().unwrap().trim_start_matches("  Starting items: ");
            let items = items.split(", ").map(|x| x.parse().unwrap()).collect();
            let op = it.next().unwrap().trim_start_matches("  Operation: new = ");
            let op = op.parse().unwrap();
            let div = it
                .next()
                .unwrap()
                .trim_start_matches("  Test: divisible by ");
            let div = div.parse().unwrap();
            let is = it
                .next()
                .unwrap()
                .trim_start_matches("    If true: throw to monkey ");
            let is = is.parse().unwrap();
            let not = it
                .next()
                .unwrap()
                .trim_start_matches("    If false: throw to monkey ");
            let not = not.parse().unwrap();
            it.next(); // skip line.

            m.push(Monkey {
                items,
                op,
                test: Test { div, is, not },
                inspected: 0,
            });
        }
        Self(m)
    }
}
impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let (_, op, v) = (it.next(), it.next(), it.next());
        match (op.unwrap(), v.unwrap()) {
            ("*", "old") => Ok(Operation::Square),
            ("*", x) => Ok(Operation::Mult(x.parse().unwrap())),
            ("+", x) => Ok(Operation::Add(x.parse().unwrap())),
            _ => unreachable!(),
        }
    }
}
