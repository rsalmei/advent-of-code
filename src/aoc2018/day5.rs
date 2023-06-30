use crate::Input;
use std::{iter, mem};

pub fn run(input: Input) {
    let data = Vec::from(input.lines().next().unwrap());

    // part one.
    fn react(w: &[u8], out: &mut Vec<u8>) {
        w.iter()
            .chain(iter::once(&0))
            .fold(None, |acc: Option<u8>, &d| {
                match acc {
                    Some(a) if a.to_ascii_lowercase() != d.to_ascii_lowercase() => out.push(a),
                    Some(a) if a.is_ascii_lowercase() ^ d.is_ascii_lowercase() => return None,
                    Some(a) => out.push(a),
                    None => {}
                };
                Some(d)
            });
    }
    fn react_fully(mut w: Vec<u8>) -> usize {
        let mut out = Vec::with_capacity(w.len());
        loop {
            react(&w, &mut out);
            match w.len() == out.len() {
                true => break out.len(),
                false => {
                    mem::swap(&mut w, &mut out);
                    out.clear();
                }
            }
        }
    }
    println!("{}", react_fully(data.clone()));

    // part two.
    let min_len = (b'a'..=b'z')
        .map(|unit| {
            let unit_up = unit.to_ascii_uppercase();
            let polymer = data
                .iter()
                .filter(|&&c| c != unit && c != unit_up)
                .copied()
                .collect();
            react_fully(polymer)
        })
        .min();
    println!("{}", min_len.unwrap());
}
