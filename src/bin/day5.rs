use std::io;
use std::io::prelude::*;

fn react(mut units: Vec<char>) -> String {
    let mut again = true;
    while again {
        again = false;
        let mut i = 0;
        while i + 1 < units.len() {
            let c = units[i];
            let d = units[i+1];
            if c != d && c.to_ascii_lowercase() == d.to_ascii_lowercase() {
                units.remove(i);
                units.remove(i);
                again = true;
            } else {
                i += 1;
            }
        }
    }
    units.iter().collect()
}


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer)?;

    let units: Vec<char> = buffer.trim_right().chars().collect();

    let mut uniq: Vec<char> = units.iter().map(|c| c.to_ascii_lowercase()).collect();
    uniq.sort();
    uniq.dedup();

    let result = react(units);
    println!("Part 1: {} {}", result.len(), result);

    let mut best = ('_', result.len());
    for c in uniq {
        println!("'{}'", c);
        let removed = result.chars().filter(|d| d.to_ascii_lowercase() != c).collect();
        let result = react(removed);
        println!("'{}': {} {}", c, result.len(), result);

        if result.len() < best.1 {
            best = (c, result.len());
        }
    }
    println!("{} {}", best.0, best.1);
    Ok(())
}
