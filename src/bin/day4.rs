use std::{fs::read_to_string, io::Error, ops::RangeInclusive};
type Section = RangeInclusive<u32>;

fn pair(line: &str) -> (Section, Section) {
    fn parse(s: &str) -> Section {
        let (start, end) = s.split_once('-').unwrap();
        start.parse().unwrap()..=end.parse().unwrap()
    }
    let (a, b) = line.split_once(',').unwrap();
    (parse(a), parse(b))
}

fn overlaps_full((a, b): &&(Section, Section)) -> bool {
    a.contains(&b.start()) && a.contains(&b.end()) || b.contains(&a.start()) && b.contains(&a.end())
}

fn overlaps_any((a, b): &&(Section, Section)) -> bool {
    a.contains(&b.start()) || a.contains(&b.end()) || b.contains(&a.start()) || b.contains(&a.end())
}

fn main() -> Result<(), Error> {
    let pairs: Vec<_> = read_to_string("./input/4.txt")?.lines().map(pair).collect();

    let count_a = pairs.iter().filter(overlaps_full).count();
    println!("A: {:?}", count_a);

    let count_b = pairs.iter().filter(overlaps_any).count();
    println!("B: {:?}", count_b);

    Ok(())
}
