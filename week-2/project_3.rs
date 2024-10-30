fn main(){
    
    let p:f64=510_000.0;
    let r:f32=5.0/100.0;
    let n:u8=3;
    let bct:f64= 1.0-f64::from(r);
    let a = p * bct.powf(f64::from(n));
    
    println!("The value of the TV after 3 years is {}",a);
    
    
}
