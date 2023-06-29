use crate::Input;
use std::collections::HashSet;
use std::iter;

pub fn run(input: Input) {
    let data = input
        .lines()
        .flat_map(|s| {
            let (direction, steps) = s.split_once(' ').unwrap();
            let steps = steps.parse::<usize>().unwrap();
            iter::repeat(direction).take(steps)
        })
        .collect::<Vec<_>>();

    fn sim(data: &[&str], tail_size: usize) -> usize {
        let mut head = (0i32, 0i32);
        let mut rope = vec![(0, 0); tail_size];
        let mut visited = HashSet::new();
        for &s in data {
            match s {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => unreachable!(),
            }
            rope.iter_mut().fold(head, |head, tail| {
                if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                    tail.0 += (head.0 - tail.0).clamp(-1, 1);
                    tail.1 += (head.1 - tail.1).clamp(-1, 1);
                }
                *tail
            });
            visited.insert(rope[tail_size - 1]);
            // println!("{s} -> H {head:?}  T {tail:?} ({})", visited.len());
        }
        visited.len()
    }

    // part one.
    println!("{}", sim(&data, 1));

    // part two.
    println!("{}", sim(&data, 9));
}
