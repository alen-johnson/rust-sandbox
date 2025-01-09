#[allow(unused_variables)]
use std::io;
use std::thread;
use std::time::Duration;
mod operations;

fn main() {
 loop{

    println!("Calculate");
    println!("1. Add \n2. Subtract \n3. Multiply \n4. Divide \n5. Power \n6. Square Root \n7. Exponential \n8. Logarithm \n9. Exit");
    let mut in_choice = String::new();
    io::stdin().read_line(&mut in_choice).expect("Failed to read line");

    let choice: u32 = match in_choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a valid choice.");
            thread::sleep(Duration::from_secs(1));
            continue;
        }
    };

    if choice == 9 {
        println!("Exiting the program. Goodbye!");
        break;
    }
    println!("Enter two numbers");
    let mut in1 = String::new();
    let mut in2 = String::new();

    io::stdin().read_line(&mut in1).expect("Failed to read line");
    io::stdin().read_line(&mut in2).expect("Failed to read line");

    let n1: f64 = match in1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input for the first number.");
            thread::sleep(Duration::from_secs(1));
            continue;
        }
    };

    let n2: f64 = match in2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input for the second number.");
            thread::sleep(Duration::from_secs(1));
            continue;
        }
    };

   match choice{
    1 => operations::add::add(n1,n2),
    2 => operations::subtract::subtract(n1,n2),
    3 => operations::multiply::multiply(n1,n2),
    4 => operations::divide::divide(n1,n2),
    5 => operations::power::power(n1,n2),
    6 => operations::sqrt::sqrt(n1,n2),
    7 => operations::exponential::exponential(n1,n2),
    8 => operations::log10::log10(n1,n2),
    _ => println!("Invalid input"),
   }

   thread::sleep(Duration::from_secs(1));
}
}
