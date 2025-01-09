pub fn sqrt(n1: f64, n2: f64){
    if n1 < 0.0 {
        println!("Cannot calculate square root of negative numbers")
    }else{
    println!("==========================");
    println!("√{} = {}", n1, n1.sqrt() );
    println!("√{} = {}", n2, n2.sqrt() );
    println!("==========================");
    }
}