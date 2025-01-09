use rand::Rng;
use std::cmp::Ordering; 
use std::io;


fn main() {
    
    let secret_num = rand::thread_rng().gen_range(1..100);

    println!("Welcome to Guess the number!");
    println!("Guess a number b/w 1 and 100");

    loop{
        println!("Input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => {
                if secret_num - guess > 10{
                println!("Too Small")
                } else{
                    println!("Small")
                }
            },
            Ordering::Greater => {
                if guess - secret_num > 10{
                println!("Too Big")
                } else{
                    println!("Big")
                }
            },
            Ordering::Equal => {
                println!("Correct Number ");
                break;
        }
        }

        // if guess < secret_num {
        //     if secret_num - guess > 10 {
        //         println!("Too small!");
        //     } else {
        //         println!("Small!");
        //     }
        // } else if guess > secret_num {
        //     if guess - secret_num > 10 {
        //         println!("Too big!");
        //     } else {
        //         println!("Big!");
        //     }
        // } else {
        //     println!("correct guess!");
        //     break;
        // }


    }
}
