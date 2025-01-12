#[allow(unused_imports)]
use std::io;
use std::time::{Duration,Instant};
use std::thread;
use std::process::Command;

use crate::clear::clear;

pub fn stopwatch() {

    clear();
    println!("Press 'Enter' to start" );

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let start = Instant::now();
    println!("Stopwatch Started. Press 'Enter' to stop");

    loop {
        let mut i = String::new();
        io::stdin().read_line(&mut i).expect("Failed to read line");

        if i.trim().is_empty() {
            let elapsed = start.elapsed();
            println!("Elapsed time : {} seconds", elapsed.as_secs());
            thread::sleep(Duration::from_secs(2));
            break;
        }
    }
}