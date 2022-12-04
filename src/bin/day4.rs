use std::{fs::read_to_string, io::Error};
type Section = (u32, u32);

fn pair(line: &str) -> (Section, Section) {
    fn parse(s: &str) -> Section {
        let (start, end) = s.split_once('-').unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    }
    let (a, b) = line.split_once(',').unwrap();
    (parse(a), parse(b))
}

fn overlaps_full((a, b): &&(Section, Section)) -> bool {
    (a.0..=a.1).contains(&b.0) && (a.0..=a.1).contains(&b.1)
        || (b.0..=b.1).contains(&a.0) && (b.0..=b.1).contains(&a.1)
}

fn overlaps_any((a, b): &&(Section, Section)) -> bool {
    (a.0..=a.1).contains(&b.0)
        || (a.0..=a.1).contains(&b.1)
        || (b.0..=b.1).contains(&a.0)
        || (b.0..=b.1).contains(&a.1)
}

fn main() -> Result<(), Error> {
    let pairs: Vec<_> = read_to_string("./input/4.txt")?.lines().map(pair).collect();

    let count_a = pairs.iter().filter(overlaps_full).count();
    println!("A: {:?}", count_a);

    let count_b = pairs.iter().filter(overlaps_any).count();
    println!("B: {:?}", count_b);

    Ok(())
}
