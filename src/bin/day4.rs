use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_sleepiest_minute(when: [usize; 60]) -> (usize, usize) {
    let (frequency, min) = when.iter().enumerate().map(|(x, y)| (y, x)).max().unwrap();
    return (min, *frequency);
}

fn main() {
    let stdin = io::stdin();
    let mut lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    lines.sort_unstable();

    // current guard
    let mut guard: u32 = 0;
    let mut sleep_start_mins: usize = 0;
    let mut guard_sleep = HashMap::new();
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
            let (e, f) = guard_sleep.entry(guard).or_insert((0, [0; 60]));
            *e += mins - sleep_start_mins;
            for min in sleep_start_mins..mins {
                (*f)[min] += 1;
            }
        }
    }

    /* TODO: hey, I guess I should learn how to write a higher-order function */
    /* part 1 */
    let (guard, (total, when)) = guard_sleep.iter().max_by_key(|(_, (total, _))| total).unwrap();
    let (sleepiest_minute, _) = get_sleepiest_minute(*when);
    println!("Guard #{} slept {} mins, mostly on min {} -> {}", guard, total, sleepiest_minute, *guard as usize * sleepiest_minute);

    /* part 2 */
    let (guard, (total, when)) = guard_sleep.iter().max_by_key(|(_, (_, when))| get_sleepiest_minute(*when).1).unwrap();
    let (sleepiest_minute, _) = get_sleepiest_minute(*when);
    println!("Guard #{} slept {} mins, mostly on min {} -> {}", guard, total, sleepiest_minute, *guard as usize * sleepiest_minute);
}
