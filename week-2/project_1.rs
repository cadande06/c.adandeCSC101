fn main(){
    let p:f64=520_000_000.0;
    let r:f32=10.0/100.0;
    let n:u8=5;
    let bct:f64= 1.0+f64::from(r);
    let a = p * bct * f64::from(n);
    let ci:f64 = a - p;
    println!("{}",a);
    println!("{}", ci);
    
}
