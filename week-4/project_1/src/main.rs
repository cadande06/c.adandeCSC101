use std::io;

fn main() {
    // Get the values of a, b, c
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    // For a
    println!("Enter the coefficient of x^2 (a):");
    io::stdin().read_line(&mut a).expect("Failed to read input for a");
    let a: f32 = a.trim().parse().expect("Not a valid number for a");

    // For b
    println!("Enter the coefficient of x (b):");
    io::stdin().read_line(&mut b).expect("Failed to read input for b");
    let b: f32 = b.trim().parse().expect("Not a valid number for b");

    // For c
    println!("Enter the constant (c):");
    io::stdin().read_line(&mut c).expect("Failed to read input for c");
    let c: f32 = c.trim().parse().expect("Not a valid number for c");

    // Assign signs based on values
    let sign_a = if a < 0.0 { "-" } else { "" };
    let sign_b = if b < 0.0 { "-" } else { "+" };
    let sign_c = if c < 0.0 { "-" } else { "+" };

    // Print out the equation with correct formatting
    println!("The equation is {}{}x^2 {} {}x {} {} = 0",sign_a,a.abs(),sign_b,b.abs(),sign_c,c.abs());
    
    //CALCULATIONS
    let di=(b*b)-(4.0*a*c);
    
    
    if di>0.0{
        
        
        let root_1=(-b+di.sqrt())/(2.0*a);
        let root_2=(-b-di.sqrt())/(2.0*a);
        
        println!("The values of x are {root_1} and {root_2}.");
    }else if di==0.0{
        
        let root=-b/(2.0*a);
        
        println!("The value of x is {root}.");
    }else{
        println!("There is no real root.");
    }

}
