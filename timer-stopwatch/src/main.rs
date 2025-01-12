use std::io;
use std::thread;
use std::time::Duration;

mod timer;
mod stopwatch;
mod clear;

fn main() {
    
    loop{
    clear::clear();
    println!("======= Menu =======");
    println!("1. Timer");
    println!("2. Stopwatch");
    println!("3. Exit");
    
    let c = get_user_choice();

    match c {
        1 => timer::timer(),
        2 => stopwatch::stopwatch(),
        3 => {
            println!("Exiting the Program, Bye");
            thread::sleep(Duration::from_secs(1));
        break;
        },
        _ => {
            println!("Invalid choice, try again");
            continue;
        }
    }
   thread::sleep(Duration::from_secs(1));

    }
}


fn get_user_choice() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input, enter a number");
        0
    })
}