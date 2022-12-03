use std::{io::Error, fs::read_to_string};

fn main() -> Result<(), Error> {
    let entries = read_to_string("./input/1.txt")?
        .lines()
        .map(|s| s.parse().ok())
        .collect::<Vec<Option<u32>>>();

    let mut sums = entries
        .split(|x| x.is_none())
        .map(|e| e.iter().map(|c| c.unwrap()).sum::<u32>())
        .collect::<Vec<_>>();

    let max = sums.iter().max().unwrap();
    println!("A: {:?}", max);

    sums.sort();
    sums.reverse();
    let sum: u32 = sums.iter().take(3).sum();
    println!("B: {:?}", sum);

    Ok(())
}
