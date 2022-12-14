use std::{fs::read_to_string, io::Error};
type Instruction = (usize, usize, usize);
type Stacks = [Vec<char>; 9];

fn instruction(line: &str) -> Instruction {
    let nums: Vec<_> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    (nums[0], nums[1] - 1, nums[2] - 1)
}

fn method_a(mut stacks: Stacks, &(n, src, dst): &Instruction) -> Stacks {
    let mut items = stacks[src].split_off(stacks[src].len() - n);
    items.reverse();
    stacks[dst].extend(items);
    stacks
}

fn method_b(mut stacks: Stacks, &(n, src, dst): &Instruction) -> Stacks {
    let items = stacks[src].split_off(stacks[src].len() - n);
    stacks[dst].extend(items);
    stacks
}

fn top_crates(
    initial: &Stacks,
    instructions: &[Instruction],
    rearrange_method: impl Fn(Stacks, &Instruction) -> Stacks,
) -> String {
    instructions
        .iter()
        .fold(initial.clone(), rearrange_method)
        .iter()
        .filter_map(|s| s.last())
        .collect()
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/5.txt")?;
    let mut stacks: Stacks = Default::default();
    for line in input.lines().take(8) {
        for (i, c) in line.char_indices().filter(|(_, c)| c.is_alphabetic()) {
            stacks[(i - 1) / 4].insert(0, c);
        }
    }
    let instructions: Vec<_> = input.lines().skip(10).map(instruction).collect();

    let crates_a = top_crates(&stacks, &instructions, method_a);
    println!("A: {}", crates_a);

    let crates_b = top_crates(&stacks, &instructions, method_b);
    println!("B: {}", crates_b);

    Ok(())
}
