use crate::Input;

pub fn run(input: Input) {
    let data = input.as_parse_optional::<u32>();
    let mut elf_cal = data
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|x| x.unwrap()).sum())
        .collect::<Vec<_>>();

    // part one.
    println!("{}", elf_cal.iter().max().unwrap());

    // part two.
    elf_cal.sort_unstable_by_key(|&c| -(c as i32));
    println!("{}", elf_cal.iter().take(3).sum::<u32>());
}
