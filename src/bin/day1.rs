use std::{fs::read_to_string, io::Error};

fn main() -> Result<(), Error> {
    let entries: Vec<_> = read_to_string("./input/1.txt")?
        .lines()
        .map(|s| s.parse::<u32>().ok())
        .collect();

    let mut sums: Vec<_> = entries
        .split(|x| x.is_none())
        .map(|e| e.iter().map(|c| c.unwrap()).sum::<u32>())
        .collect();

    let max = sums.iter().max().unwrap();
    println!("A: {:?}", max);

    sums.sort();
    sums.reverse();
    let sum: u32 = sums.iter().take(3).sum();
    println!("B: {:?}", sum);

    Ok(())
}
