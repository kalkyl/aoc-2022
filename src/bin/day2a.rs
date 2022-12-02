use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Error;

fn val(c: char) -> u32 {
    match c {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        _ => 3,
    }
}

fn score((a, b): &(u32, u32)) -> u32 {
    if a == b {
        return 3 + b;
    }
    let winner_by_loser: HashMap<u32, u32> = [(1, 2), (2, 3), (3, 1)].into_iter().collect();
    if winner_by_loser.get(&a) == Some(&b) {
        6 + b
    } else {
        0 + b
    }
}

fn main() -> Result<(), Error> {
    let entries: Vec<_> = read_to_string("./input/2.txt")?
        .lines()
        .map(|l| {
            (
                val(l.chars().nth(0).unwrap()),
                val(l.chars().nth(2).unwrap()),
            )
        })
        .collect();

    let total: u32 = entries.iter().map(score).sum();
    println!("{:?}", total);
    Ok(())
}
