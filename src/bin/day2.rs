use std::io;
use std::io::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut twos = HashSet::new();
    let mut threes = HashSet::new();
    let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    for (i, line) in lines.iter().enumerate() {
        let mut counts = HashMap::new();
        for c in line.chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }
        println!("{}", line);
        for (_c, count) in &counts {
            match count {
                2 => twos.insert(i),
                3 => threes.insert(i),
                _ => true,
            };
        }
    }
    println!("{} * {} = {}", twos.len(), threes.len(), twos.len() * threes.len());

    for xs in &lines {
        for ys in &lines {
            let mut diffs = HashSet::new();
            for (i, (x, y)) in xs.chars().zip(ys.chars()).enumerate() {
                if x != y {
                    diffs.insert(i);
                }
            }
            if diffs.len() == 1 {
                let i = diffs.iter().next().unwrap();
                let (h, t) = xs.split_at(*i);
                let (_, u) = t.split_at(1);
                println!("{}{}", h, u);
            }
        }
    }
}
