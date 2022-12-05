use std::{fs::read_to_string, io::Error};
type Instruction = (usize, usize, usize);
type Stacks = [Vec<char>; 9];

fn instruction(s: &str) -> Instruction {
    let nums: Vec<_> = s
        .split_whitespace()
        .filter_map(|y| y.parse::<usize>().ok())
        .collect();
    (nums[0], nums[1], nums[2])
}

fn rearrange_a(mut stacks: Stacks, &(n, src, dst): &Instruction) -> Stacks {
    for _ in 0..n {
        if let Some(item) = stacks[src - 1].pop() {
            stacks[dst - 1].push(item);
        }
    }
    stacks
}

fn rearrange_b(mut stacks: Stacks, &(n, src, dst): &Instruction) -> Stacks {
    let items = stacks[src - 1].split_off(stacks[src - 1].len() - n);
    stacks[dst - 1].extend(items);
    stacks
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

    let tops_a: String = instructions
        .iter()
        .fold(stacks_init.clone(), rearrange_a)
        .iter()
        .filter_map(|x| x.last())
        .collect();
    println!("A: {:?}", tops_a);

    let tops_b: String = instructions
        .iter()
        .fold(stacks_init, rearrange_b)
        .iter()
        .filter_map(|x| x.last())
        .collect();
    println!("B: {:?}", tops_b);

    Ok(())
}
