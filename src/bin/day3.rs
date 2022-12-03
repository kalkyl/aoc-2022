use std::{fs::read_to_string, io::Error};

fn find_item(bag: &str) -> char {
    let (first, second) = bag.split_at(bag.len() / 2);
    first
        .chars()
        .find(|&a| second.chars().any(|b| b == a))
        .unwrap()
}

fn find_badge(group: &[&str]) -> char {
    group[0]
        .chars()
        .find(|&a| group[1].chars().any(|b| b == a) && group[2].chars().any(|c| c == a))
        .unwrap()
}

fn prio(c: char) -> u32 {
    match c.is_lowercase() {
        true => c as u32 - 96,
        false => c as u32 - 64 + 26,
    }
}

fn main() -> Result<(), Error> {
    let bags = read_to_string("./input/3.txt")?;
    let bags: Vec<_> = bags.lines().collect();

    let total_a: u32 = bags.iter().map(|b| prio(find_item(b))).sum();
    println!("A: {:?}", total_a);

    let total_b: u32 = bags.chunks(3).map(|g| prio(find_badge(g))).sum();
    println!("B: {:?}", total_b);
    Ok(())
}
