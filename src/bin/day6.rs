use std::{collections::HashSet, fs::read_to_string, io::Error};

fn find_marker(input: &str, len: usize) -> Option<usize> {
    fn is_marker(window: &[char]) -> bool {
        let unique: HashSet<_> = window.iter().collect();
        unique.len() == window.len()
    }
    let input: Vec<_> = input.chars().collect();
    input.windows(len).position(is_marker).map(|i| i + len)
}

fn main() -> Result<(), Error> {
    let input = read_to_string("./input/6.txt")?;
    println!("A: {}", find_marker(&input, 4).unwrap());
    println!("B: {}", find_marker(&input, 14).unwrap());
    Ok(())
}
