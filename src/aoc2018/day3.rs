use crate::Input;
use std::str::FromStr;

pub fn run(input: Input) {
    let data = input.as_parse_type::<Claim>();

    // part one.
    let overlap = (0..1000)
        .flat_map(|x| (0..1000).map(move |y| (x, y)))
        .map(|(x, y)| {
            data.iter()
                .try_fold(true, |acc, c| {
                    (!c.contains(x, y))
                        .then_some(acc)
                        .or_else(|| acc.then_some(false))
                })
                .map_or(1, |_| 0)
        })
        .sum::<u32>();
    println!("{}", overlap);

    // part two.
    let unique = data.iter().find(|&c| {
        data.iter()
            .filter(|&c2| !std::ptr::eq(c, c2))
            .all(|c2| !c.overlaps(c2))
    });
    println!("{}", unique.unwrap().id);
}

#[derive(Debug)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    x2: u32,
    y2: u32,
}

impl Claim {
    fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x <= self.x2 && y >= self.y && y <= self.y2
    }

    fn overlaps(&self, c: &Claim) -> bool {
        (self.x..=self.x2)
            .flat_map(|x| (self.y..=self.y2).map(move |y| (x, y)))
            .any(|(x, y)| c.contains(x, y))
    }
}

impl FromStr for Claim {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // #1 @ 1,3: 4x4
        let mut it = s
            .split(&['#', ' ', '@', ',', ':', 'x'])
            .filter_map(|x| x.parse().ok());
        let (id, x, y) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
        Ok(Claim {
            id,
            x,
            y,
            x2: x + it.next().unwrap() - 1,
            y2: y + it.next().unwrap() - 1,
        })
    }
}
