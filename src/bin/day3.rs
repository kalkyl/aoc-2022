use std::{fs::read_to_string, io::Error};

fn find_item(input: &str) -> char {
    let (first, second) = input.split_at(input.len() / 2);
    first
        .chars()
        .find(|&a| second.chars().any(|b| b == a))
        .unwrap()
}

fn find_badge(input: &[&str]) -> char {
    input[0]
        .chars()
        .find(|&a| input[1].chars().any(|b| b == a) && input[2].chars().any(|c| c == a))
        .unwrap()
}

fn prio(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 96,
        false => c as u32 - 64 + 26,
    }
}

fn main() -> Result<(), Error> {
    let entries = read_to_string("./input/3.txt")?;
    let entries: Vec<_> = entries.lines().collect();

    let total_a: u32 = entries.iter().map(|l| prio(find_item(l))).sum();
    println!("A: {:?}", total_a);

    let total_b: u32 = entries.chunks(3).map(|l| prio(find_badge(l))).sum();
    println!("B: {:?}", total_b);
    Ok(())
}
