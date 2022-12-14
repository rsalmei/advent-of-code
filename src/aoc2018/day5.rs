use crate::Input;
use std::borrow::Cow;
use std::iter;

pub fn run(input: Input) {
    let mut data = input.data.into_bytes();
    data.pop(); // \n at the end of the file.

    // part one.
    let react = |data: &[u8]| {
        let mut out = Vec::with_capacity(data.len());
        data.iter()
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
        out
    };
    let react_fully = |data: &[u8]| {
        let mut before = Cow::Borrowed(data);
        loop {
            let after: Cow<[u8]> = Cow::Owned(react(&before));
            match before.len() == after.len() {
                true => break after,
                false => before = after,
            }
        }
    };

    let final_polymer = react_fully(&data);
    println!("{}", final_polymer.len());

    // part two.
    let min_len = ('a' as u8..'z' as u8)
        .into_iter()
        .map(|unit| [unit, unit.to_ascii_uppercase()])
        .map(|units| {
            let polymer = data
                .iter()
                .filter(|&c| !units.contains(c))
                .copied()
                .collect::<Vec<_>>();
            react_fully(&polymer).len()
        })
        .min();
    println!("{}", min_len.unwrap());
}
