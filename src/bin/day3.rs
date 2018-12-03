use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Claim {
    elf: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

fn parse_claim(line: String) -> Claim {
    let at = line.find('@').unwrap();
    let comma = line.find(',').unwrap();
    let colon = line.find(':').unwrap();
    let cross = line.find('x').unwrap();

    let &elf = &line[1..at - 1].parse::<u32>().unwrap();
    let &x = &line[at + 2..comma].parse::<usize>().unwrap();
    let &y = &line[comma + 1..colon].parse::<usize>().unwrap();
    let &w = &line[colon + 2..cross].parse::<usize>().unwrap();
    let &h = &line[cross + 1..].parse::<usize>().unwrap();

    return Claim { elf, x, y, w, h };
}

impl Claim {
    fn xs(&self) -> std::ops::Range<usize> {
        return self.x..(self.x+self.w);
    }

    fn ys(&self) -> std::ops::Range<usize> {
        return self.y..(self.y+self.h);
    }
}

fn main() {
    let mut fabric = [[0; 1000]; 1000];

    let stdin = io::stdin();
    let claims: Vec<Claim> = stdin.lock().lines().map(|l| l.unwrap()).map(parse_claim).collect();
    for r in &claims {
        for i in r.xs() {
            for j in r.ys() {
                fabric[i][j] += 1;
            }
        }
    }

    // flat_map! this feels just like home!
    println!("{}", fabric.iter().flat_map(|row| row.iter()).filter(|&x| *x > 1).count());
    for r in &claims {
        // annoyingly fabric[r.xs()][r.ys()] doesn't mean what I want it to
        if fabric[r.xs()].iter().flat_map(|row| row[r.ys()].iter()).filter(|&x| *x != 1).count() == 0 {
            println!("{}", r.elf);
        }
    }
}
