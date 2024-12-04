fn main() {
    // using Vec::new
    let v : Vec<i64> = Vec::new();
    println!("\nThe length of Vec::new is: {}",v.len());

    // using vec!
    let u = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];
    println!("\nThe length of vec macro is: {}",u.len());
}
