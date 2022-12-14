use crate::Input;

pub fn run(input: Input) {
    let data = input.as_lines();
    let mut elf_cal = data
        .iter()
        .chain([&""])
        .scan(0, |acc, &x| {
            if x.is_empty() {
                let res = *acc;
                *acc = 0;
                Some(Some(res))
            } else {
                *acc += x.parse::<u32>().unwrap();
                Some(None)
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    // part one.
    println!("{}", elf_cal.iter().max().unwrap());

    // part two.
    elf_cal.sort_unstable_by_key(|&c| -(c as i32));
    println!("{}", elf_cal.iter().take(3).sum::<u32>());
}
