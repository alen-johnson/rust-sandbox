use std::io;
use std::thread;
use std::time::Duration;
use std::process::Command;

fn is_palindrome(s: &str) -> bool {
    
    let i: String = s.chars().filter(|c| c.is_alphanumeric()).collect();

    let i = i.to_lowercase();

    return i == i.chars().rev().collect::<String>();
}

fn main() {
    println!("Palindrome Checker");

    loop{
        println!("----------------");
        println!("Enter a string or a word");
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read");

        let s = s.trim();

        if is_palindrome(s){
            println!("{} is a palindrome", s);
        }else{
            println!("{} is not a palindrome", s);
        }

        println!("----------------");
        println!("Do you want to check another one? (yes/no)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        let input = input.trim().to_lowercase();

        if input == "no" || input == "nah" || input == "n" || input == "na"{
            break;
        }
        if cfg!(target_os = "windows") {
            Command::new("cmd").arg("/C").arg("cls").status().expect("Failed to clear screen");
        } else {
            Command::new("clear").status().expect("Failed to clear screen");
        }

    }
    println!("Exiting the program");
    thread::sleep(Duration::from_secs(1));

}

