use std::{fs::read_to_string, io::Error};

fn pair(line: &str) -> ((u32, u32), (u32, u32)) {
    fn parse(s: &str) -> (u32, u32) {
        let (start, end) = s.split_once('-').unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    }
    let (a, b) = line.split_once(',').unwrap();
    (parse(a), parse(b))
}

fn overlaps_all((a, b): &&((u32, u32), (u32, u32))) -> bool {
    (a.0..=a.1).contains(&b.0) && (a.0..=a.1).contains(&b.1)
        || (b.0..=b.1).contains(&a.0) && (b.0..=b.1).contains(&a.1)
}

fn overlaps_any((a, b): &&((u32, u32), (u32, u32))) -> bool {
    (a.0..=a.1).contains(&b.0) || (a.0..=a.1).contains(&b.1)
        || (b.0..=b.1).contains(&a.0) || (b.0..=b.1).contains(&a.1)
}

fn main() -> Result<(), Error> {
    let pairs: Vec<_> = read_to_string("./input/4.txt")?.lines().map(pair).collect();

    let count_a = pairs.iter().filter(overlaps_all).count();
    println!("A: {:?}", count_a);

    let count_b = pairs.iter().filter(overlaps_any).count();
    println!("B: {:?}", count_b);

    Ok(())
}
