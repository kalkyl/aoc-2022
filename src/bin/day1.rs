use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let entries = BufReader::new(File::open("./input/1.txt")?)
        .lines()
        .map(|l| l.map(|v| v.parse().ok()))
        .collect::<Result<Vec<Option<u32>>, _>>()?;

    let mut sums = entries
        .split(|x| x.is_none())
        .map(|e| e.iter().map(|c| c.unwrap()).sum::<u32>())
        .collect::<Vec<_>>();

    // A
    println!("{:?}", sums.iter().max());

    // B
    sums.sort();
    sums.reverse();
    let sum: u32 = sums.iter().take(3).sum();
    println!("{:?}", sum);

    Ok(())
}
