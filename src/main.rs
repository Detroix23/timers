//! # `Timer`.
//! src/main.rs
//! 
//! Terminal time tools for focusing.

use std::time::{self, Duration};
use std::thread;

mod cli;

fn test_time1() -> () {
    const SECOND: Duration = Duration::from_secs(1);
    
    let mut seconds: u64 = 0;
    let mut timer = time::Duration::from_secs(seconds);

    while timer.as_secs() < 5 {
        timer += SECOND;
        thread::sleep(SECOND);
        println!("{}s", timer.as_secs());
    }
    
    seconds = timer.as_secs();
    
    println!("{}s", seconds);
}

/// Main entry point.
fn main() -> () {
    println!("# Timer.");

    
}
