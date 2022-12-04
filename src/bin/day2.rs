use std::{fs::read_to_string, io::Error};

const RULES: [(u32, u32); 3] = [(1, 2), (2, 3), (3, 1)];

fn winner(loser: &u32) -> u32 {
    RULES.iter().find(|(l, _)| l == loser).unwrap().1
}

fn loser(winner: &u32) -> u32 {
    RULES.iter().find(|(_, w)| w == winner).unwrap().0
}

fn vals(line: &str) -> (u32, u32) {
    fn val(c: &str) -> u32 {
        match c {
            "A" | "X" => 1,
            "B" | "Y" => 2,
            _ => 3,
        }
    }
    let (a, b) = line.split_once(' ').unwrap();
    (val(a), val(b))
}

fn score_a((a, b): &(u32, u32)) -> u32 {
    if a == b {
        return 3 + b;
    }
    match winner(a) == *b {
        true => 6 + b,
        false => 0 + b,
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
    let rounds: Vec<_> = read_to_string("./input/2.txt")?.lines().map(vals).collect();

    let total_a: u32 = rounds.iter().map(score_a).sum();
    println!("A: {:?}", total_a);

    let total_b: u32 = rounds.iter().map(score_b).sum();
    println!("B: {:?}", total_b);

    Ok(())
}
