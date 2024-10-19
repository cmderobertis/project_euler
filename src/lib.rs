use std::time::{Duration, Instant};

pub fn run(problem: &fn() -> i32) {
    let now: Instant = Instant::now();
    let time_delta: Duration;
    let return_value: i32 = problem();

    time_delta = now.elapsed();

    print!("{}", return_value);
    println!(" - {:?}", time_delta);
}