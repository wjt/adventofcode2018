use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    lines.sort_unstable();

    // current guard
    let mut guard: u32 = 0;
    let mut sleep_start_mins: usize = 0;
    let mut guard_sleep_total = HashMap::new();
    let mut guard_sleep_minutes = HashMap::new();
    for line in lines {
        let mins: usize = (&line[15..17]).parse().unwrap();
        let verb = &line[19..24];
        println!("{} {}", verb, mins);
        if verb == "Guard" {
            guard = (&line[26..]).split(' ').next().unwrap().parse().unwrap();
        } else if verb == "falls" {
            sleep_start_mins = mins;
        } else {
            println!("{} slept {}..{}", guard, sleep_start_mins, mins);
            *guard_sleep_total.entry(guard).or_insert(0) += mins - sleep_start_mins;
            let min_arr = guard_sleep_minutes.entry(guard).or_insert([0; 60]);
            for min in sleep_start_mins..mins {
                (*min_arr)[min] += 1;
            }
        }
    }
    /* part 1 */
    /* didn't bother with max_by(value), just read it off... */
    for (g, t) in guard_sleep_total {
        let v = guard_sleep_minutes.get(&g).unwrap();
        let (_, q) = v.iter().enumerate().map(|(x, y)| (y, x)).max().unwrap();
        println!("{} slept {} mins, mostly on min {}", g, t, q);
    }
}
