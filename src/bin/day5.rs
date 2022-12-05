use std::{fs::read_to_string, io::Error};
type Instruction = (usize, usize, usize);
type Stacks = [Vec<char>; 9];

fn instruction(s: &str) -> Instruction {
    let nums: Vec<_> = s
        .split_whitespace()
        .filter_map(|y| y.parse::<usize>().ok())
        .collect();
    (nums[0], nums[1] - 1, nums[2] - 1)
}

fn rearrange_a(mut stacks: Stacks, &(n, src, dst): &Instruction) -> Stacks {
    for _ in 0..n {
        if let Some(item) = stacks[src].pop() {
            stacks[dst].push(item);
        }
    }
    stacks
}

fn rearrange_b(mut stacks: Stacks, &(n, src, dst): &Instruction) -> Stacks {
    let items = stacks[src].split_off(stacks[src].len() - n);
    stacks[dst].extend(items);
    stacks
}

fn top_crates(
    init: &Stacks,
    instructions: &[Instruction],
    f: impl Fn(Stacks, &Instruction) -> Stacks,
) -> String {
    instructions
        .iter()
        .fold(init.clone(), f)
        .iter()
        .filter_map(|x| x.last())
        .collect()
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/5.txt")?;
    let mut stacks_init: Stacks = Default::default();
    for line in input.lines().take(8) {
        for (i, c) in line.char_indices().filter(|(_, x)| x.is_alphabetic()) {
            stacks_init[i / 4].insert(0, c);
        }
    }
    let instructions: Vec<Instruction> = input.lines().skip(10).map(instruction).collect();

    let crates_a = top_crates(&stacks_init, &instructions, rearrange_a);
    println!("A: {}", crates_a);

    let crates_b = top_crates(&stacks_init, &instructions, rearrange_b);
    println!("B: {}", crates_b);

    Ok(())
}
