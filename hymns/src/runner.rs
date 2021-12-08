use std::fmt::Display;
use std::time::Instant;

pub fn timed_run<T: Display, F: FnOnce() -> T>(part_num: u8, func: F) {
    let start = Instant::now();
    let result = func();
    let duration = Instant::now() - start;

    println!("part {}: {}", part_num, result);
    println!(
        "part {} took {}ms ({}µs)",
        part_num,
        duration.as_millis(),
        duration.as_micros()
    );
}
