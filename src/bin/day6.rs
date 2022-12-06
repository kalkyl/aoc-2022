use std::{collections::HashSet, fs::read_to_string, io::Error};

fn is_marker(window: &&[(usize, char)]) -> bool {
    let unique: HashSet<_> = window.iter().map(|(_, c)| c).collect();
    unique.len() == window.len()
}

fn main() -> Result<(), Error> {
    let input: Vec<_> = read_to_string("./input/6.txt")?.char_indices().collect();
    let packet = input.windows(4).find(is_marker).unwrap().last().unwrap().0 + 1;
    println!("A: {:?}", packet);
    let message = input.windows(14).find(is_marker).unwrap().last().unwrap().0 + 1;
    println!("B: {:?}", message);
    Ok(())
}
