use std::{fs::read_to_string, io::Error};

const RULES: [(u32, u32); 3] = [(1, 2), (2, 3), (3, 1)];

fn winner(loser: &u32) -> u32 {
    RULES.iter().find(|(l, _)| l == loser).unwrap().1
}

fn loser(winner: &u32) -> u32 {
    RULES.iter().find(|(_, w)| w == winner).unwrap().0
}

fn val(c: &str) -> u32 {
    match c {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        _ => 3,
    }
}

fn score_a((a, b): &(u32, u32)) -> u32 {
    if a == b {
        return 3 + b;
    }
    if winner(a) == *b {
        6 + b
    } else {
        0 + b
    }
}

fn score_b((a, b): &(u32, u32)) -> u32 {
    match b {
        2 => 3 + a,
        3 => 6 + winner(a),
        _ => 0 + loser(a),
    }
}

fn main() -> Result<(), Error> {
    let entries: Vec<_> = read_to_string("./input/2.txt")?
        .lines()
        .map(|l| l.split_once(' ').map(|(a, b)| (val(a), val(b))).unwrap())
        .collect();

    let total_a: u32 = entries.iter().map(score_a).sum();
    println!("A: {:?}", total_a);

    let total_b: u32 = entries.iter().map(score_b).sum();
    println!("B: {:?}", total_b);
    Ok(())
}
