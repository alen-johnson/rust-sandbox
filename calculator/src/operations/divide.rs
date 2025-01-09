pub fn divide(n1: f64, n2: f64) {
    if n2 == 0.0{
        println!("Divison by Zero not possible");
        std::process::exit(1);
    }
    println!("==========================");
    println!("{} / {} = {}", n1, n2, n1 / n2);
    println!("==========================");
}