use std::io;

fn main() -> io::Result<()> {
    let mut data = aoc2018::input(5)?.chars().collect::<Vec<_>>();
    data.pop();

    // part one.
    let react = |d: &[char], offset| {
        d.iter()
            .take(offset)
            .chain(
                d[offset..]
                    .chunks(2)
                    .flat_map(|d| match d {
                        [a, b] if a.to_ascii_lowercase() != b.to_ascii_lowercase() => Some(d),
                        [a, b] => (!a.is_ascii_lowercase() ^ b.is_ascii_lowercase()).then(|| d),
                        _ => Some(d),
                    })
                    .flatten(),
            )
            .copied()
            .collect::<Vec<_>>()
    };
    let react_fully = |mut start: Vec<char>| loop {
        let end = react(&react(&start, 0), 1);
        match start.len() == end.len() {
            true => break end,
            false => start = end,
        }
    };

    let final_polymer = react_fully(data.clone());
    println!("{}", final_polymer.len());

    // part two.
    let min_len = ('a'..'z')
        .into_iter()
        .map(|unit| [unit, unit.to_ascii_uppercase()])
        .map(|units| {
            let polymer = data
                .iter()
                .filter(|&c| !units.contains(c))
                .copied()
                .collect::<Vec<_>>();
            react_fully(polymer).len()
        })
        .min();
    println!("{}", min_len.unwrap());
    Ok(())
}
