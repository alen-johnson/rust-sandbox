pub fn log10(n1: f64, n2: f64){
    println!("==========================");
    
    if n1 <= 0.0 {
        println!("log10({}) is not defined for non-positive numbers.", n1);
    } else {
        println!("log10({}) = {}", n1, n1.log10());
    }
    
    if n2 <= 0.0 {
        println!("log10({}) is not defined for non-positive numbers.", n2);
    } else {
        println!("log10({}) = {}", n2, n2.log10());
    }

    println!("==========================");

}