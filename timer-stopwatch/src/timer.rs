use std::io;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use crate::clear::clear;


pub fn timer() {
    loop {
        clear();
        println!("Enter time in seconds:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let time: u64 = input.trim().parse().unwrap_or_else(|_| {
            0
        });

        if time > 0 {
            let start = Instant::now();
            let duration = Duration::new(time, 0);
            clear();
            println!("Timer started for {} seconds.", time);

            while start.elapsed() < duration {
                thread::sleep(Duration::from_secs(1));
                clear();
                let elapsed = start.elapsed().as_secs();
                let remaining = time - elapsed;
                println!("Remaining: {} seconds", remaining);
            }
            println!("Time's up!");
            thread::sleep(Duration::from_secs(2));
            break; 
        } else {
            clear();
            println!("Please enter a valid number greater than 0.");
            thread::sleep(Duration::from_secs(1));

        }
    }
}
